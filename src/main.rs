mod error;

use crate::error::FindPastPaperError;
use axum::{
    body::boxed,
    extract::Form,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::{
    net::SocketAddr,
    process::{Command, Output},
    thread,
};
use tokio::sync::mpsc::{channel, Sender};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), FindPastPaperError> {
    let app = Router::new()
        .route("/", get(root).post(match_input))
        .nest_service("/js", ServeDir::new("assets/js"))
        .nest_service("/css", ServeDir::new("assets/css"));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    match axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        Ok(i) => Ok(i),
        Err(_) => Err(FindPastPaperError::ServerFailed),
    }
}

async fn root() -> Html<&'static str> {
    Html(std::include_str!("../assets/home.html"))
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    user_input: String,
    subject: String,
}

fn scheduler(question: String, subject: String, tx: Sender<Output>) {
    let reply = pdfgrep(question, subject);
    tx.blocking_send(reply.unwrap()).unwrap();
}

fn pdfgrep(question: String, subject: String) -> Result<Output, std::io::Error> {
    let text = Command::new("pdfgrep")
        .args([
            &question,
            &("past-papers/".to_string().to_owned() + &subject),
            "-n",
            "-r",
            "-H",
            "--match-prefix-separator",
            "|",
        ])
        .output();
    return text;
}

async fn match_input(Form(input): Form<Input>) -> impl IntoResponse {
    let (tx, mut rx) = channel(100);
    if input.user_input.is_empty() {
        return Err(FindPastPaperError::InputFieldIsEmpty
            .to_string()
            .into_response()
            .map(boxed));
    } else if input.user_input.len() < 3 {
        return Err(FindPastPaperError::UnsuitableInputLength
            .to_string()
            .into_response()
            .map(boxed));
    };
    let question = input.user_input.clone();
    let subject = input.subject.clone();

    thread::spawn(move || scheduler(question, subject, tx));

    let text = rx.recv().await;

    match text {
        Some(t) => {
            let result = format_output_for_html_display(input.subject, input.user_input, t);
            Ok(result.into_response().map(boxed))
        }
        _ => Err(FindPastPaperError::CouldNotReadFile
            .to_string()
            .into_response()
            .map(boxed)),
    }
}

// ensure the output is displayed on the html page like: "[DIRECTORY] - page [PAGE NUMBER] - <i>[OUTPUT]</i>\n" per line
fn format_output_for_html_display(subject: String, question: String, output: Output) -> String {
    let mut output_string = String::from_utf8(output.stdout).unwrap();

    let no_of_lines = output_string
        .matches("\n")
        .count()
        .to_string()
        .parse()
        .unwrap();

    let even_numbers = get_even(no_of_lines);
    let mut even_prefix_sep_indices: Vec<usize> = Vec::new();

    for i in even_numbers {
        let even_prefix_sep_index = get_char_index_of_nth_separator(output_string.clone(), i);
        even_prefix_sep_indices.push(even_prefix_sep_index);
    }

    // replace the nth instances (in which n is an even number) with "~"
    for i in even_prefix_sep_indices {
        output_string.replace_range(i..i + 1, "~")
    }

    let final_result = output_string
        .replace("~", " - page ")
        .replace("|", " - <i>...")
        .replace("\n", "...</i>\n\n")
        .replace(&question, &("<b>".to_owned() + &question + "</b>"))
        .replace(&("past-papers/".to_owned() + &subject + &"/"), "");

    return final_result;
}

fn get_char_index_of_nth_separator(output_string: String, n: u32) -> usize {
    return output_string
        .match_indices("|")
        .nth(n.try_into().unwrap())
        .map(|s| s.0)
        .unwrap();
}

fn get_even(number: u32) -> Vec<u32> {
    let mut even_numbers: Vec<u32> = Vec::new();
    for i in 0..number * 2 {
        if i % 2 == 0 {
            even_numbers.push(i);
        } else {
            continue;
        }
    }

    return even_numbers;
}
