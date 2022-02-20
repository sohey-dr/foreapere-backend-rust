mod handler;

use actix_web::{web, App, HttpServer};

#[actix_rt::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handler::index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
