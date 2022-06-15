use std::sync::Arc;

use axum::{response::Html, routing::*, Extension, Router};
use tera::Tera;
#[tokio::main]
async fn main() {
    let tera = Tera::new("templates/**/*").expect("Could not initialize Tera template engine.");
    let app = Router::new()
        .route("/", get(index))
        .layer(Extension(Arc::new(tera)));
    // Listen on port 3000
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index(Extension(tera): Extension<Arc<Tera>>) -> Html<String> {
    let text = tera
        .render("index.html", &tera::Context::new())
        .expect("Could not render HTML");
    Html(text)
}
