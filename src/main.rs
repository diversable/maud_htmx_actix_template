use actix_files::Files;
use actix_web::{get, post, App, HttpServer, Result as ActixResult};
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};

fn head(title: &str) -> Markup {
    let stylesheet = "/public/stylesheet.css";

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                (Css(stylesheet))

                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) }
                (PreEscaped
                    ("<script src=\"https://unpkg.com/htmx.org@1.9.8\"></script>")
                )
            }

        }
    }
}

fn footer() -> Markup {
    html! {
        footer .footer {
            "This is a footer"
        }
    }
}

#[get("/")]
async fn index() -> ActixResult<Markup> {
    Ok(html! {
        (head("Web Page Title"))

        body hx-boost {
            h1 {"Getting Started"}
        }

        (footer())
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("The server is running at localhost:8000");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(Files::new("/public", "./public").show_files_listing())
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

/// A shorthand for including CSS Stylesheets
///
/// Links to a CSS Stylesheet at the given path
struct Css(&'static str);

impl Render for Css {
    fn render(&self) -> Markup {
        html! {
            link rel="stylesheet" type="text/css" href=(self.0);
        }
    }
}
