use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder,
    cookie::Cookie,
    get,
    http::header::LOCATION,
    web::{self, Redirect},
};
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

#[derive(Deserialize)]
pub struct DiscordUser {
    id: String,
    username: String,
    global_name: Option<String>,
    avatar: Option<String>,
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
        .basic_auth(env::var("CLIENT_ID")?, Some(env::var("CLIENT_SECRET")?))
        .send()
        .await?;

    if !res.status().is_success() {
        return Ok(None);
    }

    let text = res.text().await?;
    let json = serde_json::from_str(&text)?;
    return Ok(json);
}

pub async fn get_user(token: &str) -> Result<Option<DiscordUser>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://discord.com/api/v10/users/@me")
        .header("Authorization", format!("Bearer {token}"))
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
async fn index(
    query: web::Query<IndexQuery>,
    req: HttpRequest,
) -> Result<HttpResponse, Box<dyn Error>> {
    if let Some(code) = query.code.as_deref() {
        let token = get_token(code).await?;
        if let Some(access_token) = token.map(|token| token.access_token) {
            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/"))
                .cookie(
                    Cookie::build("access_token", access_token)
                        .secure(true)
                        .http_only(true)
                        .finish(),
                )
                .finish())
        } else {
            Ok(HttpResponse::InternalServerError().finish())
        }
    } else if let Some(token) = req
        .cookie("access_token")
        .as_ref()
        .map(|cookie| cookie.value())
    {
        let res = get_user(token).await?;
        if let Some(user) = res {
            Ok(HttpResponse::Ok().body(format!(
                "hello, {}",
                user.global_name.as_deref().unwrap_or(&user.username)
            )))
        } else {
            Ok(HttpResponse::InternalServerError().finish())
        }
    } else {
        Ok(HttpResponse::Ok().body("hello stranger"))
    }
}

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
