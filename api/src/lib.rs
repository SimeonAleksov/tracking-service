use service::{
    sea_orm::{Database, DatabaseConnection},
};
use migration::{
    Migrator, MigratorTrait
};
use actix_web::{
    middleware, web, App, HttpServer,
};

use listenfd::ListenFd;
use std::env;


#[actix_web::main]
async fn start() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "full");

    dotenvy::dotenv().ok();
    let db_url = "postgresql://debug:debug@localhost:5432/tracking";
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // create post table if not exists
    let conn: DatabaseConnection = Database::connect(db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default()) // enable logger
            .configure(init)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting server at {server_url}");
    server.run().await?;

    Ok(())
}

fn init(_cfg: &mut web::ServiceConfig) {
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}")
    }
}