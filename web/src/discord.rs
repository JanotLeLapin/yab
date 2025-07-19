use serde::Deserialize;
use std::{env, error::Error};

#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Deserialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
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
