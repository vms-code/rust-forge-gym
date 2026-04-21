use axum::{routing::{get, post}, Router};
use tower_http::services::ServeDir;
use std::sync::Arc;
use sqlx::SqlitePool;
use dotenv;
use clap::Parser;
use serde_json;
use std::fs;

mod db;
mod models;
mod handlers;
mod render;
mod error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum Command {
    /// Build quizzes manifest from source files
    Build,

    /// Run the web server
    Serve,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<SqlitePool>,
    pub quizzes: Arc<models::QuizzesManifest>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let cmd = Command::parse();

    match cmd {
        Command::Build => {
            println!("🔨 Building quizzes manifest...");
            render::build().expect("Failed to build quizzes");
            println!("✅ Manifest generated successfully: quizzes.json");
        }

        Command::Serve => {
            // Load quizzes manifest ONCE at startup
            let manifest_content = fs::read_to_string("quizzes.json")
                .expect("quizzes.json not found, run `cargo run -- build` first");

            let quizzes: models::QuizzesManifest = serde_json::from_str(&manifest_content)
                .expect("Failed to parse quizzes.json");

            println!("📚 Loaded {} quizzes from manifest", quizzes.len());

            let pool = Arc::new(
                SqlitePool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set"))
                    .await
                    .expect("Failed to connect to DB"),
            );

            sqlx::migrate!()
                .run(&*pool)
                .await
                .expect("Failed to run migrations");

            let state = AppState {
                db: pool,
                quizzes: Arc::new(quizzes),
            };

            let app = Router::new()
                .route("/", get(handlers::home))
                .route("/quiz", get(handlers::get_random_quiz))
                .route("/answer", post(handlers::submit_answer))
                .nest_service("/static", ServeDir::new("static"))
                .with_state(state);

            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
                .await
                .expect("Failed to bind to port 3000");

            println!("🚀 Rust Forge Gym running at http://localhost:3000");
            axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        }
    }
}
