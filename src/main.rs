use axum::{
    body::boxed,
    extract::Form,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::{
    fmt,
    net::SocketAddr,
    process::{Command, Output},
    thread,
};
use tokio::sync::mpsc::{channel, Sender};
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
            Error::InputFieldIsEmpty => write!(f, "Please enter a question in the input field"),
            Error::UnsuitableInputLength => write!(f, "Input must be over 3 characters"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
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
    let question = input.user_input.clone();
    let subject = input.subject;

    thread::spawn(move || scheduler(question, subject, tx));

    let text = rx.recv().await;

    match text {
        Some(t) => {
            let result = format_output_for_html_display(input.user_input, t);
            Ok(result.into_response().map(boxed))
        }
        _ => Err(Error::CouldNotReadFile
            .to_string()
            .into_response()
            .map(boxed)),
    }
}

// ensure the output is displayed on the html page like: "[DIRECTORY] - page [PAGE NUMBER] - <i>[OUTPUT]</i>\n" per line
fn format_output_for_html_display(question: String, output: Output) -> String {
    let mut output_string = String::from_utf8(output.stdout).unwrap();

    // let new_string = output_string.replace("\n", "...</i> \n");
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

    let formatted_page_numbers = output_string.replace("~", " - page ");
    let added_first_italics_tags = formatted_page_numbers.replace("|", " - <i>...");
    let close_italics_tags = added_first_italics_tags.replace("\n", "...</i>\n");
    let final_result = close_italics_tags.replace(
        &question,
        &("<b>".to_string().to_owned() + &question + &"</b>".to_string()),
    );

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
