use maud::{Markup, html};

pub fn layout(title: &str, style: Option<&str>, body: Markup) -> Markup {
    html! {
        head {
            title { (title) }
            script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.6/dist/htmx.min.js" {}
            style { (grass::include!("web/style/core.scss")) }
            @match style {
                Some(v) => style { (v) },
                None => {},
            }
        }
        body {
            (body)
        }
    }
}
