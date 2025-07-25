use actix_web::{HttpRequest, get};
use maud::{Markup, html};
use std::error::Error;

use crate::discord;

#[get("/_fragments/profile")]
pub async fn profile(req: HttpRequest) -> Result<Markup, Box<dyn Error>> {
    let user = if let Some(token) = req.cookie("access_token") {
        match discord::get_user(token.value()).await {
            Ok(Some(user)) => Some(user),
            _ => None,
        }
    } else {
        None
    };

    if let Some(user) = user {
        Ok(html! {
            @if let Some(avatar) = user.avatar {
                img src={ "https://cdn.discordapp.com/avatars/" (user.id) "/" (avatar) ".png?size=32" };
            }
            p { (user.global_name.as_deref().unwrap_or(&user.username)) }
        })
    } else {
        Ok(html! {
            a { "Login" }
        })
    }
}

pub fn component() -> Markup {
    html! {
        header {
            h3 { "yab" }
            .profile hx-get="/_fragments/profile" hx-trigger="load" {}
        }
    }
}
