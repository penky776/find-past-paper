use axum::{
    body::boxed,
    extract::Form,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::process::Command;
use std::{fmt, net::SocketAddr};

#[derive(Debug)]
enum Error {
    CouldNotReadFile,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CouldNotReadFile => write!(f, "Could not read file"),
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root).post(match_input));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {
    Html(std::include_str!("../assets/home.html"))
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    user_input: String,
}

async fn match_input(Form(input): Form<Input>) -> impl IntoResponse {
    let input = input.user_input;
    let text = Command::new("pdfgrep")
        .args([&input, "past papers/", "-n", "-r", "-H"])
        .output();

    match text {
        Ok(t) => {
            let string_text = String::from_utf8(t.stdout).unwrap();
            Ok(string_text.into_response().map(boxed))
        }
        Err(_) => Err(Error::CouldNotReadFile
            .to_string()
            .into_response()
            .map(boxed)),
    }
}
