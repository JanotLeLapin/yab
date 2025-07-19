use actix_web::{App, HttpServer};

mod discord;

mod pages;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    HttpServer::new(|| App::new().service(pages::index::page))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
