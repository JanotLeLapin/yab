use actix_web::{App, HttpResponse, HttpServer, cookie::Cookie, get, web};
use serde::Deserialize;
use std::{env, error::Error};

#[derive(Deserialize)]
struct IndexQuery {
    code: Option<String>,
}

#[derive(Deserialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: usize,
    refresh_token: String,
    scope: String,
}

pub async fn get_token(code: &str) -> Result<Option<TokenResponse>, Box<dyn Error>> {
    let client = reqwest::ClientBuilder::new().build()?;
    let mut data = std::collections::HashMap::new();
    data.insert("grant_type", "authorization_code");
    data.insert("code", code);
    data.insert("redirect_uri", "http://localhost:8080");

    let res = client
        .post(&format!("https://discord.com/api/v10/oauth2/token"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&data)
        .basic_auth("1395042202702315580", Some(env::var("CLIENT_SECRET")?))
        .send()
        .await?;

    if !res.status().is_success() {
        return Ok(None);
    }

    let text = res.text().await?;
    let json = serde_json::from_str(&text)?;
    return Ok(json);
}

#[get("/")]
async fn index(query: web::Query<IndexQuery>) -> Result<HttpResponse, Box<dyn Error>> {
    let mut res = HttpResponse::Ok();
    if let Some(code) = query.code.as_deref() {
        let token = get_token(code).await?;
        if let Some(access_token) = token.map(|token| token.access_token) {
            res.cookie(
                Cookie::build("access_token", access_token)
                    .secure(true)
                    .http_only(true)
                    .finish(),
            );
        }
    }

    Ok(res.body("hello, world!"))
}

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
