use fastly::http::StatusCode;
use fastly::{mime, Error, Request, Response};
mod board;
use std::collections::HashMap;
use std::str;

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    let url = req.get_url().clone();

    let params: HashMap<String, String> = req.get_query()?;

    match url.path() {
        "/" => Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::TEXT_HTML_UTF_8)
            .with_body(str::from_utf8(include_bytes!("static/index.html")).unwrap())),
        "/next" => {
            let mut b = board::Board::new();

            b.read_from_string(params["board"].as_str()).unwrap();

            match b.pick() {
                Ok(status) => {
                    let body: String = match status {
                        board::ActionStatus::AIWon => String::from(" I WIN   "),
                        board::ActionStatus::PlayerWon => String::from("YOU   WIN"),
                        _ => b.get_board_string(),
                    };

                    Ok(Response::from_status(StatusCode::OK)
                        .with_content_type(mime::TEXT_HTML_UTF_8)
                        .with_body(format!("{}", body)))
                },
                Err(err) => Ok(Response::from_status(StatusCode::INTERNAL_SERVER_ERROR)
                    .with_content_type(mime::TEXT_HTML_UTF_8)
                    .with_body(format!("{}", format!("{:?}", err)))),
            }
        }
        _ => {
            return Ok(Response::from_status(StatusCode::NOT_FOUND)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body("NOT FOUND"));
        }
    }
}
