use std::sync::Arc;

use axum::{
    extract::Form,
    response::{Html, IntoResponse},
    routing::*,
    Extension, Router,
};
use serde::{Deserialize, Serialize};
use tera::Tera;

#[derive(Deserialize, Serialize)]
pub enum Hours {
    Morning,
    Afternoon,
    Evening,
}
#[derive(Deserialize, Serialize)]
pub struct GymForm {
    first_name: String,
    surname: String,
    email: String,
    phone: String,
    frequency: i32,
    mon: Option<bool>,
    tue: Option<bool>,
    wed: Option<bool>,
    thu: Option<bool>,
    fri: Option<bool>,
    sat: Option<bool>,
    sun: Option<bool>,
    hours: Hours,
    interests: String,
    legal_bs: bool,
}

#[tokio::main]
async fn main() {
    let tera = Tera::new("templates/**/*").expect("Could not initialize Tera template engine.");
    let app = Router::new()
        .route("/", get(index))
        .route("/process", post(process))
        .layer(Extension(Arc::new(tera)));
    // Listen on port 3000
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index(Extension(tera): Extension<Arc<Tera>>) -> impl IntoResponse {
    let text = tera
        .render("index.html", &tera::Context::new())
        .expect("Could not render HTML");
    Html(text)
}

async fn process(
    Extension(tera): Extension<Arc<Tera>>,
    Form(gym): Form<GymForm>,
) -> impl IntoResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("form", &gym);
    let mut weekdays_string = String::new();
    if gym.mon.is_some() {
        weekdays_string += "Poniedziałek ";
    }
    if gym.tue.is_some() {
        weekdays_string += "Wtorek ";
    }
    if gym.wed.is_some() {
        weekdays_string += "Środa ";
    }
    if gym.thu.is_some() {
        weekdays_string += "Czwartek ";
    }
    if gym.fri.is_some() {
        weekdays_string += "Piątek ";
    }
    if gym.sat.is_some() {
        weekdays_string += "Sobota ";
    }
    if gym.sun.is_some() {
        weekdays_string += "Niedziela ";
    }
    ctx.insert("weekdays", &weekdays_string);
    ctx.insert(
        "hours",
        &match gym.hours {
            Hours::Afternoon => "popołudniowe",
            Hours::Evening => "wieczorne",
            Hours::Morning => "poranne",
        },
    );
    Html(tera.render("process.html", &ctx).unwrap())
}
