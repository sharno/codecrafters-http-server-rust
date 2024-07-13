pub mod http;
mod server;

use std::{net::TcpListener, sync::Arc};

use http::{Header, Request, Response, Status};
use itertools::Itertools;
use server::process;

#[tokio::main]
async fn main() {
    let routes = Arc::new(vec![
        ("/", index_handler as fn(Request) -> Response),
        ("/echo/{str}", echo_handler as fn(Request) -> Response),
        ("/user-agent", user_agent_handler as fn(Request) -> Response),
    ]);

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                let routes = Arc::clone(&routes);
                tokio::spawn(async move { process(&mut stream, routes) });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn echo_handler(req: Request) -> Response {
    let path_parts = req.path.split("/").collect_vec();
    let body = path_parts[2].to_owned();
    return Response {
        status: Status::Ok,
        headers: vec![
            Header::new("Content-Type", "text/plain"),
            Header::new("Content-Length", &body.len().to_string()),
        ],
        body: body,
    };
}

fn index_handler(_req: Request) -> Response {
    return Response {
        status: Status::Ok,
        headers: vec![],
        body: "".to_owned(),
    };
}

fn user_agent_handler(req: Request) -> Response {
    let body = req
        .headers
        .iter()
        .find(|h| h.name == "User-Agent")
        .unwrap()
        .value
        .to_owned();
    return Response {
        status: Status::Ok,
        headers: vec![
            Header::new("Content-Type", "text/plain"),
            Header::new("Content-Length", &body.len().to_string()),
        ],
        body: body,
    };
}
