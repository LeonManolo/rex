use crate::http_request_decoder::HttpRequest;
use crate::request::Request;
use crate::response::Response;
use regex::Regex;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct RexApp {
    routes: Vec<Route>,
}

type RouteCallback = fn(request: Request, response: &mut Response) -> ();

struct Route {
    method: String,
    path: String,
    path_regex: Regex,
    callback: RouteCallback,
    // order is important!
    params: Vec<String>,
}

impl RexApp {
    pub fn new() -> Self {
        Self { routes: vec![] }
    }

    pub fn get(&mut self, path: &str, function: RouteCallback) {
        self.push_route("GET", path, function);
    }

    pub fn post(&mut self, path: &str, function: RouteCallback) {
        self.push_route("POST", path, function);
    }

    pub fn delete(&mut self, path: &str, function: RouteCallback) {
        self.push_route("DELETE", path, function);
    }

    // TODO: other http methods here

    fn push_route(&mut self, method: &str, path: &str, function: RouteCallback) {
        let regex_path = RexApp::prepare_path_regex(path);
        let regex = Regex::new(&regex_path.to_string()).expect("Valid regex required!");
        println!("regex: {}", regex_path);

        let mut params = vec![];
        for regex_group_name in regex.capture_names() {
            // if regex_group_name != None {
            //     let group_name = regex_group_name.unwrap();
            // }

            // Äquivalent!

            if let Some(group_name) = regex_group_name {
                params.push(group_name.to_string());
            }
        }

        let route = Route {
            method: method.to_string(),
            path: path.to_string(),
            path_regex: Regex::new(regex_path.as_str()).unwrap(), // TODO: unsafe unwrap!
            callback: function,
            params: params,
        };

        self.routes.push(route);
    }

    fn prepare_path_regex(path: &str) -> String {
        let mut regex_string = String::from("^");

        let url_segments = path.split("/");

        for url_segment in url_segments {
            if url_segment.contains(":") {
                let value = url_segment.trim_start_matches(':');
                regex_string.push_str(&format!("/(?<{}>[^/]+)", value));
            } else if !url_segment.is_empty() {
                regex_string.push_str(&format!("/{}", url_segment));
            }
        }

        regex_string.push_str("$");
        regex_string
    }

    fn find_matching_route(&self, url: &str, method: &str) -> Option<&Route> {
        for route in self.routes.iter() {
            if route.path_regex.is_match(url) && route.method == method {
                return Some(&route);
            }
        }

        None
    }

    fn extract_params_from_url(url: &str, route: &Route) -> Vec<(String, String)> {
        let (leading_path, _) = url.split_once("?").unwrap_or((url, ""));
        let mut params: Vec<(String, String)> = vec![];

        if let Some(captures) = route.path_regex.captures(leading_path) {
            for param_key in route.params.clone() {
                if let Some(capture) = captures.name(&*param_key) {
                    params.push((param_key.to_string(), capture.as_str().to_string()));
                }
            }
        }

        params
    }

    pub fn listen(self, port: u16, function: fn() -> ())
    {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(address).unwrap();

        function();

        let shared_self = std::sync::Arc::new(self);

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let shared_self = std::sync::Arc::clone(&shared_self);

                    std::thread::spawn(move || {
                        let mut buffer = [0; 1024]; // TODO: vlt ändern
                        if let Err(e) = stream.read(&mut buffer) {
                            eprintln!("Failed to read from stream: {}", e);
                            return;
                        }

                        let mut response: Response;

                        if let Some(http_request) = HttpRequest::decode_from_buffer(buffer) {
                            if let Some(route) =
                                shared_self.find_matching_route(&http_request.url, &http_request.method)
                            {
                                let request = Request {
                                    method: http_request.method,
                                    url: http_request.url.clone(),
                                    http_version: http_request.version,
                                    headers: http_request.headers,
                                    params: Self::extract_params_from_url(&http_request.url, route),
                                    query_params: http_request.url_query_params,
                                    body: http_request.body,
                                };

                                response = Response::default();
                                let callback = route.callback;
                                callback(request, &mut response);
                            } else {
                                response = Response::not_found();
                            }
                        } else {
                            response = Response::error();
                        }

                        let raw_response = response.to_raw_http_response();
                        if let Err(e) = stream.write_all(raw_response.as_bytes()) {
                            eprintln!("Failed to write to stream: {}", e);
                        }
                        if let Err(e) = stream.flush() {
                            eprintln!("Failed to flush stream: {}", e);
                        }
                    });
                }
                Err(e) => eprintln!("Failed to accept connection: {}", e),
            }
        }
    }


}
