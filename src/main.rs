use askama::Template;
use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use std::sync::atomic::{AtomicUsize, Ordering};

static HELLO_INDEX: AtomicUsize = AtomicUsize::new(0);

const HELLOS: &[&str] = &[
    "Hello",        // English
    "你好",         // Chinese
    "Halo",         // Indonesian
    "こんにちは",    // Japanese
    "안녕하세요",    // Korean
    "Bonjour",      // French
    "Hola",         // Spanish
    "Ciao",         // Italian
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
    Html(template.render().unwrap())
}

async fn next_hello() -> String {
    let current = HELLO_INDEX.fetch_add(1, Ordering::Relaxed) + 1;
    let index = current % HELLOS.len();
    HELLOS[index].to_string()
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
