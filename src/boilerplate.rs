use file_ops::append_to_file;

use std::{fs, io};

pub fn add_top_boilerplate(file_path: &std::path::Path) -> Result<(), io::Error> {
    let top_boiler = r###"
use axum::{
    extract::{self, Path, Query},
    routing::{get, post},
    Json, Router,
};
use minio_rsc::{Minio, provider::StaticProvider, client::PresignedArgs};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;
use sqlx::{postgres::PgPoolOptions, prelude::FromRow};
use std::env;
use std::net::SocketAddr;
use std::result::Result;
use std::sync::Arc;
use axum::http::StatusCode;
use sqlx::types::chrono::Utc;
use std::collections::HashMap;
use tower_http::cors::{AllowOrigin, CorsLayer};
use axum::http::Method;
use reqwest;

use axum::response::{Html, IntoResponse};
use tower::service_fn;
use tower_http::services::ServeDir;


"###;
    fs::write(file_path, top_boiler)?;
    Ok(())
}

pub fn add_axum_end(funcs: Vec<String>, file_path: &std::path::Path) -> Result<(), io::Error> {
    // Ensure parent directories exist
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Create tests directory structure
    let mut routs: String = funcs
        .iter()
        .map(|func| {
            let http_method = if func.starts_with("get") {
                "get"
            } else {
                "post"
            };
            format!("\t.route(\"/{func}\", {http_method}({func}))\n").to_string()
        })
        .collect::<String>();
    routs.push_str("\t//.route(\"/signed-urls/:video_path\", get(get_signed_url))\n");
    let ending = format!(
        r###"
async fn health() -> String {{"healthy".to_string() }}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    let db_url = env::var("DATABASE_URL")
     .unwrap_or_else(|_| "postgres://dbuser:p@localhost:1111/data".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;

    let migrate = sqlx::migrate!("./migrations").run(&pool).await;

    match migrate {{
        Ok(_) => println!("Migrations applied successfully."),
        Err(e) => eprintln!("Error applying migrations: {{}}", e),
    }};

    let static_service =
        ServeDir::new("../frontend/build").not_found_service(service_fn(|_req| async {{
            match tokio::fs::read_to_string("frontend/build/index.html").await {{
                Ok(body) => Ok((StatusCode::OK, Html(body)).into_response()),
                Err(err) => Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to read index.html: {{}}", err),
                )
                    .into_response()),
            }}
        }}));


    let app = Router::new()
    .route("/health", get(health))
    {routs}
    .fallback_service(static_service)
    .layer(
        CorsLayer::new()
            .allow_origin(AllowOrigin::list(vec![
                "http://localhost:3000".parse().unwrap(),
                "https://example.com".parse().unwrap(),
            ]))
            .allow_methods([Method::GET, Method::POST])
            .allow_headers(tower_http::cors::Any)
    )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}}


"###
    ); //https://tidelabs.github.io/tidechain/tower_http/cors/struct.CorsLayer.html (may help with auth) 

    append_to_file(file_path, &ending)?;
    Ok(())
}
