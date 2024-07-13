use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

use itertools::Itertools;

const VERSION: &str = "HTTP/1.1";

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                // read the request
                let mut buf: String = String::new();
                BufReader::new(&stream).read_line(&mut buf).unwrap();
                let req = parse_req(&buf);

                // write the response
                let status = match req.path.as_str() {
                    "/" => Status::Ok,
                    _ => Status::NotFound,
                };
                let response = vec![VERSION, status.code(), status.name()].join(" ") + "\r\n\r\n";
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

struct Request {
    path: String,
}

fn parse_req(req: &str) -> Request {
    let lines = req.split("\r\n").collect_vec();
    let req_line = lines[0].split(" ").collect_vec();
    let path = req_line[1];
    return Request {
        path: path.to_owned(),
    };
}
