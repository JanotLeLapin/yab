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
    if let Some(code) = query.code.as_deref() {
        let token = discord::get_token(code).await?;
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
        let res = discord::get_user(token).await?;
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
