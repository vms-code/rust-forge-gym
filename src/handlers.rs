use axum::{response::Html, extract::{State, Query}, Form};
use askama::Template;
use crate::AppState;
use crate::models::Quiz;
use fastrand;
use chrono::Utc;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct ClassificationStats {
    pub name: String,
    pub total: usize,
    pub answered: u32,
    pub mastery: String,
    pub progress: String,
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    total_quizzes: usize,
    total_answered: u32,
    total_correct: u32,
    categories: Vec<ClassificationStats>,
    tags: Vec<ClassificationStats>,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    total_quizzes: usize,
    total_answered: u32,
    accuracy: String,
    categories: Vec<ClassificationStats>,
    tags: Vec<ClassificationStats>,
}

#[derive(Template)]
#[template(path = "quiz.html")]
struct QuizTemplate {
    pub quiz: Quiz,
    pub show_result: bool,
    pub user_answer: String,
    pub is_correct: bool,
    pub category: String,
    pub tag: String,
}

#[derive(Template)]
#[template(path = "listing.html")]
struct ListingTemplate {
    pub title: String,
    pub quizzes: Vec<QuizListItem>,
    pub category: String,
    pub tag: String,
}

#[derive(Debug, Clone)]
pub struct QuizListItem {
    pub quiz: Quiz,
    pub answered: bool,
    pub correct_attempts: i64,
}

#[derive(Deserialize, Debug)]
pub struct AnswerForm {
    quiz_id: u32,
    answer: String,
    category: Option<String>,
    tag: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct QuizFilter {
    category: Option<String>,
    tag: Option<String>,
    id: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct ResetFilter {
    category: Option<String>,
    tag: Option<String>,
}

pub async fn home(State(state): State<AppState>) -> Html<String> {
    let progress = get_all_progress(&state).await;
    
    let mut cat_map: HashMap<String, (usize, u32, u32)> = HashMap::new();
    let mut tag_map: HashMap<String, (usize, u32, u32)> = HashMap::new();
    
    let mut total_answered = 0;
    let mut total_correct = 0;

    for quiz in state.quizzes.values() {
        let (ans, corr) = progress.get(&quiz.id).map(|p| (1u32, p.correct_attempts as u32)).unwrap_or((0, 0));
        
        total_answered += ans;
        total_correct += corr;

        let cat_entry = cat_map.entry(quiz.category.clone()).or_insert((0, 0, 0));
        cat_entry.0 += 1;
        cat_entry.1 += ans;
        cat_entry.2 += corr;

        for tag in &quiz.tags {
            let tag_entry = tag_map.entry(tag.clone()).or_insert((0, 0, 0));
            tag_entry.0 += 1;
            tag_entry.1 += ans;
            tag_entry.2 += corr;
        }
    }

    let mut categories: Vec<_> = cat_map.into_iter().map(|(name, (total, answered, correct))| {
        let mastery_val = if answered > 0 { (correct as f32 / answered as f32) * 100.0 } else { 0.0 };
        let progress_val = if total > 0 { (answered as f32 / total as f32) * 100.0 } else { 0.0 };
        ClassificationStats {
            name, total, answered,
            mastery: format!("{:.1}", mastery_val),
            progress: format!("{:.1}", progress_val),
        }
    }).collect();
    categories.sort_by(|a, b| a.name.cmp(&b.name));

    let mut tags: Vec<_> = tag_map.into_iter().map(|(name, (total, answered, correct))| {
        let mastery_val = if answered > 0 { (correct as f32 / answered as f32) * 100.0 } else { 0.0 };
        let progress_val = if total > 0 { (answered as f32 / total as f32) * 100.0 } else { 0.0 };
        ClassificationStats {
            name, total, answered,
            mastery: format!("{:.1}", mastery_val),
            progress: format!("{:.1}", progress_val),
        }
    }).collect();
    tags.sort_by(|a, b| b.total.cmp(&a.total)); // Sort by popularity

    let template = HomeTemplate {
        total_quizzes: state.quizzes.len(),
        total_answered,
        total_correct,
        categories,
        tags,
    };

    Html(template.render().unwrap())
}

pub async fn dashboard(State(state): State<AppState>) -> Html<String> {
    let progress = get_all_progress(&state).await;
    
    let mut cat_map: HashMap<String, (usize, u32, u32)> = HashMap::new();
    let mut tag_map: HashMap<String, (usize, u32, u32)> = HashMap::new();
    
    let mut total_answered = 0;
    let mut total_correct = 0;

    for quiz in state.quizzes.values() {
        let (ans, corr) = progress.get(&quiz.id).map(|p| (1u32, p.correct_attempts as u32)).unwrap_or((0, 0));
        
        total_answered += ans;
        total_correct += corr;

        let cat_entry = cat_map.entry(quiz.category.clone()).or_insert((0, 0, 0));
        cat_entry.0 += 1;
        cat_entry.1 += ans;
        cat_entry.2 += corr;

        for tag in &quiz.tags {
            let tag_entry = tag_map.entry(tag.clone()).or_insert((0, 0, 0));
            tag_entry.0 += 1;
            tag_entry.1 += ans;
            tag_entry.2 += corr;
        }
    }

    let mut categories: Vec<_> = cat_map.into_iter().map(|(name, (total, answered, correct))| {
        let mastery_val = if answered > 0 { (correct as f32 / answered as f32) * 100.0 } else { 0.0 };
        let progress_val = if total > 0 { (answered as f32 / total as f32) * 100.0 } else { 0.0 };
        ClassificationStats {
            name, total, answered,
            mastery: format!("{:.1}", mastery_val),
            progress: format!("{:.1}", progress_val),
        }
    }).collect();
    categories.sort_by(|a, b| b.mastery.partial_cmp(&a.mastery).unwrap_or(std::cmp::Ordering::Equal));

    let mut tags: Vec<_> = tag_map.into_iter().map(|(name, (total, answered, correct))| {
        let mastery_val = if answered > 0 { (correct as f32 / answered as f32) * 100.0 } else { 0.0 };
        let progress_val = if total > 0 { (answered as f32 / total as f32) * 100.0 } else { 0.0 };
        ClassificationStats {
            name, total, answered,
            mastery: format!("{:.1}", mastery_val),
            progress: format!("{:.1}", progress_val),
        }
    }).collect();
    tags.sort_by(|a, b| b.mastery.partial_cmp(&a.mastery).unwrap_or(std::cmp::Ordering::Equal));

    let total_accuracy = if total_answered > 0 { (total_correct as f32 / total_answered as f32) * 100.0 } else { 0.0 };

    let template = DashboardTemplate {
        total_quizzes: state.quizzes.len(),
        total_answered,
        accuracy: format!("{:.1}", total_accuracy),
        categories,
        tags,
    };

    Html(template.render().unwrap())
}

async fn get_all_progress(state: &AppState) -> HashMap<u32, ProgressRow> {
    sqlx::query_as!(ProgressRow, "SELECT quiz_id as \"quiz_id!\", correct_attempts as \"correct_attempts!\" FROM user_progress")
        .fetch_all(&*state.db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|p| (p.quiz_id as u32, p))
        .collect()
}

struct ProgressRow {
    quiz_id: i64,
    correct_attempts: i64,
}

pub async fn get_quiz(State(state): State<AppState>, Query(filter): Query<QuizFilter>) -> Html<String> {
    let filtered_quizzes: Vec<&Quiz> = state.quizzes.values().filter(|q| {
        if let Some(cat) = &filter.category {
            if q.category != *cat { return false; }
        }
        if let Some(tag) = &filter.tag {
            if !q.tags.contains(tag) { return false; }
        }
        true
    }).collect();

    if filtered_quizzes.is_empty() {
        return Html("<h1>No quizzes found for this filter</h1><a href='/'>Go Back</a>".to_string());
    }

    let quiz = if let Some(id) = filter.id {
        filtered_quizzes.iter().find(|q| q.id == id).copied()
            .or_else(|| Some(filtered_quizzes[fastrand::usize(0..filtered_quizzes.len())]))
    } else {
        Some(filtered_quizzes[fastrand::usize(0..filtered_quizzes.len())])
    }.unwrap();

    let template = QuizTemplate {
        quiz: quiz.clone(),
        show_result: false,
        user_answer: String::new(),
        is_correct: false,
        category: filter.category.unwrap_or_default(),
        tag: filter.tag.unwrap_or_default(),
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
        category: form.category.unwrap_or_default(),
        tag: form.tag.unwrap_or_default(),
    };

    Html(template.render().unwrap())
}

pub async fn list_quizzes(State(state): State<AppState>, Query(filter): Query<QuizFilter>) -> Html<String> {
    let progress = get_all_progress(&state).await;

    let mut filtered: Vec<QuizListItem> = state.quizzes.values()
        .filter(|q| {
            if let Some(cat) = &filter.category {
                if q.category != *cat { return false; }
            }
            if let Some(tag) = &filter.tag {
                if !q.tags.contains(tag) { return false; }
            }
            true
        })
        .map(|q| {
            let p = progress.get(&q.id);
            QuizListItem {
                quiz: q.clone(),
                answered: p.is_some(),
                correct_attempts: p.map(|p| p.correct_attempts).unwrap_or(0),
            }
        })
        .collect();

    filtered.sort_by(|a, b| a.quiz.id.cmp(&b.quiz.id));

    let title = if let Some(cat) = &filter.category {
        format!("Category: {}", cat)
    } else if let Some(tag) = &filter.tag {
        format!("Tag: {}", tag)
    } else {
        "All Quizzes".to_string()
    };

    let template = ListingTemplate {
        title,
        quizzes: filtered,
        category: filter.category.unwrap_or_default(),
        tag: filter.tag.unwrap_or_default(),
    };

    Html(template.render().unwrap())
}

pub async fn reset_progress(State(state): State<AppState>, Query(filter): Query<ResetFilter>) -> Html<String> {
    if filter.category.is_none() && filter.tag.is_none() {
        // Global reset
        sqlx::query!("DELETE FROM user_progress")
            .execute(&*state.db)
            .await
            .unwrap();
    } else {
        // Filtered reset: find matching quiz IDs, then delete their progress
        let ids_to_delete: Vec<i64> = state.quizzes.values()
            .filter(|q| {
                if let Some(cat) = &filter.category {
                    if q.category != *cat { return false; }
                }
                if let Some(tag) = &filter.tag {
                    if !q.tags.contains(tag) { return false; }
                }
                true
            })
            .map(|q| q.id as i64)
            .collect();

        if !ids_to_delete.is_empty() {
            // Build placeholders for IN clause
            let placeholders: Vec<String> = (0..ids_to_delete.len()).map(|i| format!("?{}" , i + 1)).collect();
            let query_str = format!("DELETE FROM user_progress WHERE quiz_id IN ({})", placeholders.join(","));
            
            let mut query = sqlx::query(&query_str);
            for id in &ids_to_delete {
                query = query.bind(id);
            }
            query.execute(&*state.db).await.unwrap();
        }
    }

    Html(r#"<html><head><meta http-equiv="refresh" content="0; url=/" /></head></html>"#.to_string())
}
