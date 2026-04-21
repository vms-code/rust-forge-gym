use axum::{response::Html, extract::State, Form};
use askama::Template;
use crate::AppState;
use crate::models::Quiz;
use fastrand;
use chrono::Utc;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    total_quizzes: usize,
    total_answered: u32,
    total_correct: u32,
}

#[derive(Template)]
#[template(path = "quiz.html")]
struct QuizTemplate {
    pub quiz: Quiz,
    pub show_result: bool,
    pub user_answer: String,
    pub is_correct: bool,
}

#[derive(serde::Deserialize, Debug)]
pub struct AnswerForm {
    quiz_id: u16,
    answer: String,
}

pub async fn home(State(state): State<AppState>) -> Html<String> {
    let stats = sqlx::query!(
        "SELECT 
            COUNT(*) as total, 
            SUM(correct_attempts > 0) as answered,
            SUM(correct_attempts) as correct
         FROM user_progress"
    )
    .fetch_optional(&*state.db)
    .await
    .ok()
    .flatten();

    let (total_answered, total_correct) = match stats {
        Some(s) => (s.answered.unwrap_or(0) as u32, s.correct.unwrap_or(0) as u32),
        None => (0, 0),
    };

    let template = HomeTemplate {
        total_quizzes: state.quizzes.len(),
        total_answered,
        total_correct,
    };

    Html(template.render().unwrap())
}

pub async fn get_random_quiz(State(state): State<AppState>) -> Html<String> {
    let quiz = state.quizzes.values().nth(fastrand::usize(0..state.quizzes.len())).unwrap();

    let template = QuizTemplate {
        quiz: quiz.clone(),
        show_result: false,
        user_answer: String::new(),
        is_correct: false,
    };

    Html(template.render().unwrap())
}

pub async fn submit_answer(State(state): State<AppState>, Form(form): Form<AnswerForm>) -> Html<String> {
    let quiz = state.quizzes.get(&form.quiz_id).unwrap();
    let is_correct = form.answer.trim() == quiz.answer.trim();
    
    let now = Utc::now().to_rfc3339();
    let correct_val = is_correct as i32;
    let streak_val = if is_correct { 1 } else { 0 };

    sqlx::query!(
        "INSERT INTO user_progress (quiz_id, total_attempts, correct_attempts, last_attempt_at, streak)
         VALUES (?1, 1, ?2, ?3, ?4)
         ON CONFLICT(quiz_id) DO UPDATE SET
             total_attempts = total_attempts + 1,
             correct_attempts = correct_attempts + ?2,
             last_attempt_at = ?3,
             streak = CASE WHEN ?2 = 1 THEN streak + 1 ELSE 0 END",
        form.quiz_id,
        correct_val,
        now,
        streak_val
    )
    .execute(&*state.db)
    .await
    .unwrap();

    let template = QuizTemplate {
        quiz: quiz.clone(),
        show_result: true,
        user_answer: form.answer,
        is_correct,
    };

    Html(template.render().unwrap())
}