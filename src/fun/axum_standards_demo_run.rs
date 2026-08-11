use super::axum_standards_demo_logger::axum_standards_demo_logger;
use super::axum_standards_demo_router::axum_standards_demo_router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
// =====================
pub async fn axum_standards_demo_run() {
    let app = axum_standards_demo_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.expect("bind");
    axum_standards_demo_logger(addr);
    axum::serve(listener, app).await.expect("serve");
}
// =====================
