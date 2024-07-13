use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
};

use itertools::Itertools;

use crate::http::{Request, Response, Status};

pub const CRLF: &str = "\r\n";
const VERSION: &str = "HTTP/1.1";

pub fn process(stream: &mut TcpStream, routes: Arc<Vec<(&str, fn(Request) -> Response)>>) {
    // read the request
    let buf = tcp_stream_to_string(stream, 1000);
    let req = Request::parse(&buf);

    // handling the request
    let mut res: Response = Response {
        status: Status::NotFound,
        headers: vec![],
        body: "".to_owned(),
    };
    for route in routes.iter() {
        if matches(route.0, &req.path) {
            res = route.1(req);
            break;
        }
    }

    // write the response
    let response = vec![VERSION, res.status.code(), res.status.name()].join(" ")
        + CRLF
        + &res.headers.iter().map(|header| header.to_string()).join("")
        + CRLF
        + &res.body;
    println!("writing response {:#?}", response);
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn tcp_stream_to_string(stream: &mut TcpStream, max_length: usize) -> String {
    let mut buffer = vec![0; max_length];
    let bytes_read = stream.read(&mut buffer).unwrap();
    buffer.truncate(bytes_read);
    String::from_utf8(buffer).unwrap()
}

fn matches(pattern: &str, req_path: &str) -> bool {
    let pattern_parts = pattern.split("/").collect_vec();
    let path_parts = req_path.split("/").collect_vec();
    // TODO: more robust route matcher
    if pattern_parts[1] == path_parts[1] {
        return true;
    }
    return false;
}
