use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct EnvItem {
    key: &'static str,
    value: String,
}

#[derive(Serialize)]
struct EnvResponse {
    envs: Vec<EnvItem>,
}

const PUBLIC_KEYS: [&str; 4] = ["APP_NAME", "API_URL", "ENVIRONMENT", "VERSION"];
const HTML_TEMPLATE: &str = include_str!("./template.html");

fn default_for(key: &'static str) -> Option<&'static str> {
    match key {
        "APP_NAME" => Some("ServerCompass Rust Actix"),
        "API_URL" => Some("https://api.servercompass.app"),
        "ENVIRONMENT" => Some("production"),
        "VERSION" => Some("1.0.0"),
        "DATABASE_URL" => Some("postgresql://user:password@localhost:5432/servercompass"),
        "API_SECRET_KEY" => Some("your-secret-key-here"),
        _ => None,
    }
}

fn env_value(key: &'static str) -> String {
    match env::var(key) {
        Ok(v) => {
            let trimmed = v.trim();
            if trimmed.is_empty() {
                "Not set".to_string()
            } else {
                trimmed.to_string()
            }
        }
        Err(_) => default_for(key).unwrap_or("Not set").to_string(),
    }
}

fn public_envs() -> Vec<EnvItem> {
    PUBLIC_KEYS
        .iter()
        .map(|&key| EnvItem {
            key,
            value: env_value(key),
        })
        .collect()
}

#[get("/api/env")]
async fn api_env() -> impl Responder {
    web::Json(EnvResponse { envs: public_envs() })
}

#[get("/")]
async fn index() -> impl Responder {
    let app_name = env_value("APP_NAME");
    let api_url = env_value("API_URL");
    let environment = env_value("ENVIRONMENT");
    let version = env_value("VERSION");

    let class_for = |v: &str| if v == "Not set" { "not-set" } else { "" };

    let rendered = HTML_TEMPLATE
        .replace("{{APP_NAME}}", &app_name)
        .replace("{{API_URL}}", &api_url)
        .replace("{{ENVIRONMENT}}", &environment)
        .replace("{{VERSION}}", &version)
        .replace("{{APP_NAME_CLASS}}", class_for(&app_name))
        .replace("{{API_URL_CLASS}}", class_for(&api_url))
        .replace("{{ENVIRONMENT_CLASS}}", class_for(&environment))
        .replace("{{VERSION_CLASS}}", class_for(&version));

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered)
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    println!("Starting server on 0.0.0.0:{port}");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(api_env)
            .service(health)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
