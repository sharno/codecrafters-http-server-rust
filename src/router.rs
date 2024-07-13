use itertools::Itertools;

use crate::http::{Request, Response};

#[derive(Clone)]
pub struct Router {
    routes: Vec<Route>,
}
impl Router {
    pub(crate) fn new() -> Self {
        Self { routes: vec![] }
    }

    pub(crate) fn add_get(&mut self, pattern: &str, handler: fn(Request) -> Response) {
        self.routes.push(Route {
            pattern: pattern.to_owned(),
            handler: handler,
        })
    }

    pub fn get_matching_route(&self, req_path: &str) -> Option<&Route> {
        self.routes
            .iter()
            .find(|route| Self::matches(&route.pattern, req_path))
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
}

#[derive(Clone)]
pub struct Route {
    pub pattern: String,
    pub handler: fn(Request) -> Response,
}
