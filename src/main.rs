use axum::{
    body::boxed,
    extract::Form,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::{fmt, net::SocketAddr, process::Command};
use tower_http::services::ServeDir;

#[derive(Debug)]
enum Error {
    ServerFailed,
    CouldNotReadFile,
    InputFieldIsEmpty,
    UnsuitableInputLength,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ServerFailed => write!(f, "server failed to start"),
            Error::CouldNotReadFile => write!(f, "Could not read file"),
            Error::InputFieldIsEmpty => write!(f, "Input field is not allowed to be empty"),
            Error::UnsuitableInputLength => write!(f, "Input must be over 3 characters"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = Router::new()
        .route("/", get(root).post(match_input))
        .nest_service("/js", ServeDir::new("assets/js"));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    match axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        Ok(i) => Ok(i),
        Err(_) => Err(Error::ServerFailed),
    }
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
    if input.user_input.is_empty() {
        return Err(Error::InputFieldIsEmpty
            .to_string()
            .into_response()
            .map(boxed));
    } else if input.user_input.len() < 3 {
        return Err(Error::UnsuitableInputLength
            .to_string()
            .into_response()
            .map(boxed));
    };
    let input = input.user_input;
    let text = Command::new("pdfgrep")
        .args([&input, "past papers/", "-n", "-r", "-H"])
        .output();

    match text {
        Ok(t) => {
            let result = String::from_utf8(t.stdout).unwrap();
            Ok(result.into_response().map(boxed))
        }
        Err(_) => Err(Error::CouldNotReadFile
            .to_string()
            .into_response()
            .map(boxed)),
    }
}
