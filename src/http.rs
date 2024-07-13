use itertools::Itertools;

pub const CRLF: &str = "\r\n";
pub const VERSION: &str = "HTTP/1.1";

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub headers: Vec<Header>,
}

impl Request {
    pub fn parse(req: &str) -> Request {
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

    pub fn get_header(&self, name: &str) -> Option<String> {
        self.headers
            .iter()
            .find(|h| h.name == name)
            .map(|h| h.value.to_owned())
    }
}

#[derive(Debug)]
pub struct Response {
    status: Status,
    headers: Vec<Header>,
    body: String,
}

impl Response {
    pub fn ok() -> Self {
        Self {
            status: Status::OK,
            headers: Vec::new(),
            body: String::new(),
        }
    }

    pub fn not_found() -> Self {
        Self {
            status: Status::NotFound,
            headers: Vec::new(),
            body: String::new(),
        }
    }

    pub fn body(mut self, body: &str, mime_type: MimeType) -> Self {
        self.headers
            .push(Header::new("Content-Type", &mime_type.to_string()));
        self.headers
            .push(Header::new("Content-Length", &body.len().to_string()));
        self.body = body.to_owned();
        self
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", VERSION, self.status.code(), self.status.name())
            + CRLF
            + &self
                .headers
                .iter()
                .map(|header| header.to_string())
                .join("")
            + CRLF
            + &self.body
    }
}

#[derive(Debug)]
pub enum MimeType {
    TextPlain,
}

impl MimeType {
    pub fn to_string(&self) -> &str {
        match self {
            Self::TextPlain => "text/plain",
        }
    }
}

#[derive(Debug)]
pub enum Status {
    OK,
    NotFound,
}

impl Status {
    pub fn code(&self) -> &str {
        match self {
            Self::OK => "200",
            Self::NotFound => "404",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::OK => "OK",
            Self::NotFound => "Not Found",
        }
    }
}

#[derive(Debug)]
pub struct Header {
    pub name: String,
    pub value: String,
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
        return format!("{}: {}{}", self.name, self.value, CRLF);
    }
}
