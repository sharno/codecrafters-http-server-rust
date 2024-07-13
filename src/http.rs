use itertools::Itertools;

use crate::server::CRLF;

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
}

#[derive(Debug)]
pub struct Response {
    pub status: Status,
    pub headers: Vec<Header>,
    pub body: String,
}

#[derive(Debug)]
pub enum Status {
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
        return (&self).name.to_owned() + ": " + &self.value + CRLF;
    }
}
