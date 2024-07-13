use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use itertools::Itertools;

const VERSION: &str = "HTTP/1.1";
const CRLF: &str = "\r\n";

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

fn process(stream: &mut TcpStream, routes: Arc<Vec<(&str, fn(Request) -> Response)>>) {
    // read the request
    let buf = tcp_stream_to_string(stream, 1000);
    let req = parse_req(&buf);

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
    pub fn parse(line: &str) -> Self {
        let parts = line.trim().split(": ").collect_vec();
        return Header {
            name: parts[0].to_owned(),
            value: parts[1].to_owned(),
        };
    }
    pub fn to_string(&self) -> String {
        return (&self).name.to_owned() + ": " + &self.value + CRLF;
    }
}

struct Request {
    path: String,
    headers: Vec<Header>,
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

    let headers = lines
        .iter()
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| Header::parse(line))
        .collect_vec();

    return Request {
        path: path.to_owned(),
        headers: headers,
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
