use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::IntoMakeService;
use axum::Router;
use axum::Server;
use hyper::server::conn::AddrIncoming;
use std::net::TcpListener;

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello {}!", name)
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub fn run(listener: TcpListener) -> Server<AddrIncoming, IntoMakeService<Router>> {
    let app = Router::new()
        .route("/health-check", get(health_check))
        .route("/greet/:name", get(greet));

    Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
}
