use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use crate::{
    http::{Request, Response},
    router::Router,
};

pub struct Server {
    router: Router,
    port: usize,
}

impl Server {
    pub fn new(port: usize) -> Self {
        Self {
            port: port,
            router: Router::new(),
        }
    }

    pub fn get(&mut self, pattern: &str, handler: fn(Request) -> Response) {
        self.router.add_get(pattern, handler);
    }

    pub fn serve(&self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).unwrap();
        let router = Arc::new(self.router.clone());
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("accepted new connection");
                    let router = Arc::clone(&router);
                    tokio::spawn(async move { Self::process(&mut stream, router) });
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }

    fn process(stream: &mut TcpStream, router: Arc<Router>) {
        // read the request
        let buf = tcp_stream_to_string(stream, 1000);
        let req = Request::parse(&buf);

        // handling the request
        let res = router
            .get_matching_route(&req.path)
            .map(|route| (route.handler)(req))
            .unwrap_or(Response::not_found());

        // write the response
        let response = res.to_string();
        println!("writing response {:#?}", response);
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn tcp_stream_to_string(stream: &mut TcpStream, max_length: usize) -> String {
    let mut buffer = vec![0; max_length];
    let bytes_read = stream.read(&mut buffer).unwrap();
    buffer.truncate(bytes_read);
    String::from_utf8(buffer).unwrap()
}
