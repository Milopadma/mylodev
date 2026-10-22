use askama::Template;
use axum::{
    http::StatusCode,
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

#[derive(Template)]
#[template(path = "sitemark-privacy.html")]
struct SitemarkPrivacyTemplate;

// privacy policy for the sitemark phone apps; the play console and app store listings link here
async fn sitemark_privacy() -> Html<String> {
    match SitemarkPrivacyTemplate.render() {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            println!("Template rendering error: {}", e);
            Html(format!("Template error: {}", e))
        }
    }
}

#[derive(Template)]
#[template(path = "privacy-policy.html")]
struct PrivacyPolicyTemplate;

// privacy policy for the downpage iphone app; the app's plus sheet and its app store listing link here
async fn privacy_policy() -> (StatusCode, Html<String>) {
    match PrivacyPolicyTemplate.render() {
        Ok(rendered) => (StatusCode::OK, Html(rendered)),
        Err(e) => {
            println!("Template rendering error: {}", e);
            // the policy has to stay reachable, so point people at a way to ask for it
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(String::from(
                    "<p>The privacy policy couldn't load. Try again in a moment, or email <a href=\"mailto:hello@mylo.dev\">hello@mylo.dev</a> for a copy. <a href=\"/\">Go to mylo.dev</a></p>",
                )),
            )
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
        .route("/privacy-policy", get(privacy_policy))
        .route("/sitemark/privacy", get(sitemark_privacy))
        .route("/next-hello", post(next_hello));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
