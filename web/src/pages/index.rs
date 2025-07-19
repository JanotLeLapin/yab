use crate::discord;

use actix_web::{HttpRequest, HttpResponse, cookie::Cookie, get, http::header::LOCATION, web};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct IndexQuery {
    code: Option<String>,
}

#[get("/")]
pub async fn page(
    query: web::Query<IndexQuery>,
    req: HttpRequest,
) -> Result<HttpResponse, Box<dyn Error>> {
    let text = if let Some(code) = query.code.as_deref() {
        let token = discord::get_token(code).await?;
        if let Some(access_token) = token.map(|token| token.access_token) {
            return Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/"))
                .cookie(
                    Cookie::build("access_token", access_token)
                        .secure(true)
                        .http_only(true)
                        .finish(),
                )
                .finish());
        } else {
            return Ok(HttpResponse::InternalServerError().finish());
        }
    } else if let Some(token) = req
        .cookie("access_token")
        .as_ref()
        .map(|cookie| cookie.value())
    {
        let res = discord::get_user(token).await?;
        if let Some(user) = res {
            format!(
                "hello, {}",
                user.global_name.as_deref().unwrap_or(&user.username)
            )
        } else {
            return Ok(HttpResponse::InternalServerError().finish());
        }
    } else {
        "hello stranger".to_string()
    };

    Ok(HttpResponse::Ok().body(maud::html! {
        body {
            h1 { "yab" }
            p { (text) }
        }
    }))
}
