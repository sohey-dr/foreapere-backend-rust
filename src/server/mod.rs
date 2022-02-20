mod handler;

use actix_web::{web, App, HttpServer};
use diesel::{r2d2::{Pool, ConnectionManager}, mysql::MysqlConnection};
use dotenv::dotenv;
use std::env;

#[actix_rt::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(RequestContext::new())
            .route("/", web::get().to(handler::index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[derive(Clone)]
pub struct RequestContext {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl RequestContext {
    pub fn new() -> RequestContext {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create DB connection pool.");

        RequestContext { pool }
    }
}
