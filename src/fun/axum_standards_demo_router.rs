use axum::Router;
use tower_http::services::ServeDir;
// =====================
pub fn axum_standards_demo_router() -> Router {
    Router::new().fallback_service(ServeDir::new("static"))
}
// =====================
