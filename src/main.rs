use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

use itertools::Itertools;

const VERSION: &str = "HTTP/1.1";
const CRLF: &str = "\r\n";

fn main() {
    let routes = vec![
        ("/", index_handler as fn(Request) -> Response),
        ("/echo/{str}", echo_handler as fn(Request) -> Response),
    ];

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                // read the request
                let mut buf = String::new();
                BufReader::new(&stream).read_line(&mut buf).unwrap();
                let req = parse_req(&buf);

                // handling the request
                let mut res: Response = Response {
                    status: Status::NotFound,
                    headers: vec![],
                    body: "".to_owned(),
                };
                for route in &routes {
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
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

enum Status {
    Ok,
    NotFound,
}

impl Status {
    pub fn code(&self) -> &str {
        match self {
            Self::Ok => "200",
            Self::NotFound => "404",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Ok => "OK",
            Self::NotFound => "Not Found",
        }
    }
}

struct Header {
    name: String,
    value: String,
}

impl Header {
    pub fn new(name: &str, value: &str) -> Self {
        return Header {
            name: name.to_owned(),
            value: value.to_owned(),
        };
    }
    pub fn to_string(&self) -> String {
        return (&self).name.to_owned() + ": " + &self.value + CRLF;
    }
}

struct Request {
    path: String,
}

struct Response {
    status: Status,
    headers: Vec<Header>,
    body: String,
}

fn parse_req(req: &str) -> Request {
    let lines = req.split(CRLF).collect_vec();
    let req_line = lines[0].split(" ").collect_vec();
    let path = req_line[1];
    return Request {
        path: path.to_owned(),
    };
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

trait Route {
    const PATTERN: &'static str;
    fn handle(req: Request) -> Response;
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
