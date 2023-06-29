use service::{
    sea_orm::{Database, DatabaseConnection},
    Query,
};
use migration::{
    Migrator, MigratorTrait
};
use actix_web::{middleware, web, App, HttpServer, HttpResponse, Error, post};

use listenfd::ListenFd;
use std::env;
use actix_web::http::header::ContentType;
use actix_web::web::Data;
use env_logger;



#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}


#[post("/tracking/{id}/")]
async fn accounts(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let account = Query::find_accounts_by_id(conn, id).await;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(account)
    )
}


#[actix_web::main]
async fn start() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    dotenvy::dotenv().ok();
    let db_url = "postgresql://debug:debug@localhost:5432/tracking";
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn: DatabaseConnection = Database::connect(db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let state = AppState { conn };

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
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

fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(accounts);
}

pub fn main() {
    let result = start();
    if let Some(err) = result.err() {
        println!("Error: {err}")
    }
}