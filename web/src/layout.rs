use maud::{DOCTYPE, Markup, PreEscaped, html};

pub fn layout(title: &str, style: Option<&str>, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            title { (title) }
            style { (PreEscaped(grass::include!("web/style/core.scss"))) }
            @match style {
                Some(v) => style { (PreEscaped(v)) },
                None => {},
            }
            link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600;800&display=swap" rel="stylesheet";
            script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.6/dist/htmx.min.js" {}
        }
        body {
            (body)
        }
    }
}
