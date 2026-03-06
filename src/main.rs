mod api;
mod models;
pub mod progress;
mod views;

use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use api::content::ContentStore;

fn build_router() -> Router {
    let content_dir =
        std::env::var("EPICPATH_CONTENT_DIR").unwrap_or_else(|_| "content".to_string());
    let store = ContentStore::load(&content_dir);

    Router::new()
        .route("/", get(views::pages::home))
        .route("/concepts", get(views::pages::concepts_list))
        .route("/concepts/{id}", get(views::pages::concept_detail))
        .route("/concepts/{id}/complete", post(views::pages::complete_concept))
        .route("/workflows", get(views::pages::workflows_list))
        .route("/workflows/{id}", get(views::pages::workflow_detail))
        .route("/workflows/{id}/complete", post(views::pages::complete_workflow))
        .route("/paths", get(views::pages::paths_list))
        .route("/paths/{id}", get(views::pages::path_detail))
        .route("/quizzes", get(views::pages::quizzes_list))
        .route("/quizzes/{id}", get(views::pages::quiz_detail))
        .route("/quizzes/{id}/submit", post(views::pages::quiz_submit))
        .route("/search", get(views::pages::search))
        .route("/progress", get(views::pages::progress_page))
        .route("/toggle-theme", post(views::pages::toggle_theme))
        .nest_service("/static", ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(store)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = build_router();
    Ok(router.into())
}
