pub mod http;
pub mod router;
pub mod server;

use http::{Header, Request, Response, Status};
use itertools::Itertools;
use server::Server;

#[tokio::main]
async fn main() {
    let mut server = Server::new(4221);
    server.get("/", index_handler);
    server.get("/echo/{str}", echo_handler);
    server.get("/user-agent", user_agent_handler);
    server.serve();
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
