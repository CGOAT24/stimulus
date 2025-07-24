use axum::debug_handler;
use libsql::{de, Builder, Database, Rows};
use loco_rs::prelude::*;
use dotenvy::dotenv;
use std::env;
use serde_json::json;
use tracing_subscriber::fmt::format::json;
use crate::views::home::HomeResponse;

#[debug_handler]
async fn current() -> Result<Response> {
    format::json(HomeResponse::new("loco"))
}

//TODO
async fn test() -> Result<Response> {
    dotenv().expect(".env file not found");

    let url = env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL not found!");
    let token = env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN not found!");

    let mut db = Builder::new_local("local.db").build().await.unwrap();
    let conn = db.connect().unwrap();

    let mut rows: Rows = conn.query("SELECT 1", ()).await?;
    let mut result = Vec::new();
    while let Some(row) = rows.next().await {
        let json = format::json(de::from_row(&row).unwrap());
        result.push(json);
    }
    format::json(result)

}

pub fn routes() -> Routes {
    Routes::new().prefix("/api").add("/", get(current))
}
