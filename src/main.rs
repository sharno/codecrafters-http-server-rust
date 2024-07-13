pub mod http;
pub mod router;
pub mod server;

use http::{Request, Response};
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
    Response::ok().body(&body, "text/plain")
}

fn index_handler(_req: Request) -> Response {
    Response::ok()
}

fn user_agent_handler(req: Request) -> Response {
    let body = req
        .get_header("User-Agent")
        .expect("Expected to find User-Agent header for a user-agent request");
    Response::ok().body(&body, "text/plain")
}
