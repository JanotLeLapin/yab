use crate::discord;

use actix_web::{HttpRequest, HttpResponse, cookie::Cookie, get, http::header::LOCATION, web};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct IndexQuery {
    code: Option<String>,
}

#[get("/")]
pub async fn page(query: web::Query<IndexQuery>) -> Result<HttpResponse, Box<dyn Error>> {
    if let Some(code) = query.code.as_deref() {
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
    }

    Ok(HttpResponse::Ok().body(maud::html! {
        head {
            script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.6/dist/htmx.min.js" {}
            style { (grass::include!("web/style/index.scss")) }
            style { (grass::include!("web/style/core.scss")) }
        }
        body {
            header {
                h1 { "yab" }
                .profile hx-get="/api/profile" hx-trigger="load" {}
            }
            p { "Welcome to the yab home page" }
        }
    }))
}
