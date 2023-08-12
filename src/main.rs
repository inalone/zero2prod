use axum::extract::Path;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello {}!", name)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/:name", get(greet));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
