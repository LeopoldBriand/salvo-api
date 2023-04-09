use salvo::{prelude::*};
use dotenv::dotenv;

mod controllers;
mod middlewares;
mod models;
mod routes;
mod db;

#[tokio::main]
async fn main() {
    // Init utils
    dotenv().ok();
    tracing_subscriber::fmt().init();

    // Get env variables
    let address = std::env::var("API_ADDRESS").expect("API_ADDRESS must be set in .env file.");
    let port = std::env::var("API_PORT").expect("API_PORT must be set in .env file.");

    // Init database
    db::init_db().await;

    // Start server
    let router = routes::build_routes();
    tracing::info!("{}", &format!("Listening on http://{}:{}", address, port));
    Server::new(TcpListener::bind(&format!("{}:{}", address, port))).serve(router).await;
}