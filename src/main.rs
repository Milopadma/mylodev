use askama::Template;
use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::net::TcpListener;

static HELLO_INDEX: AtomicUsize = AtomicUsize::new(0);

const HELLOS: &[&str] = &[
    "Hello",      // English
    "你好",       // Chinese
    "Halo",       // Indonesian
    "こんにちは", // Japanese
    "안녕하세요", // Korean
    "Bonjour",    // French
    "Hola",       // Spanish
    "Ciao",       // Italian
];

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    hello: String,
}

async fn index() -> Html<String> {
    let template = IndexTemplate {
        hello: HELLOS[0].to_string(),
    };
    match template.render() {
        Ok(rendered) => {
            println!("Template rendered successfully");
            Html(rendered)
        }
        Err(e) => {
            println!("Template rendering error: {}", e);
            Html(format!("Template error: {}", e))
        }
    }
}

async fn next_hello() -> Html<String> {
    let current = HELLO_INDEX.fetch_add(1, Ordering::Relaxed) + 1;
    let index = current % HELLOS.len();
    let hello = HELLOS[index];

    println!("Returning hello: {}", hello);
    // return the hello text wrapped in the same h1 structure
    Html(format!("{}", hello))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/home", get(index))
        .route("/test", get(index))
        .route("/next-hello", post(next_hello));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
