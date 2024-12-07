use crate::request::Request;
use crate::response::Response;
use regex::Regex;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use crate::http_status::HttpStatus;

pub struct RexApp {
    routes: Vec<Route>,
}

type RouteCallback = fn(request: Request, response: &mut Response) -> ();

struct Route {
    method: String,
    path: String,
    path_regex: Regex,
    callback: RouteCallback,
    /// order is important!
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

    fn push_route(&mut self, method: &str, path: &str, function: RouteCallback) {
        let regex_path = RexApp::prepare_path_regex(path);
        let regex = Regex::new(&regex_path.to_string()).expect("Valid regex required!");
        println!("regex: {}", regex_path);

        let mut params = vec![];
        for regex_group_name in regex.capture_names() {
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

    fn find_matching_route(&self, path: &str, method: &str) -> Option<&Route> {
        for route in self.routes.iter() {
            if route.path_regex.is_match(path) && route.method == method {
                return Some(&route);
            }
        }

        None
    }

    // "Query params" and "params" are not the same!
    fn extract_query_params_from_url(url: &str) -> HashMap<String, String> {
        /// splits url (https://example.com/abc?param=value into https://example.com/abc and param=value)
        let mut query_params = HashMap::new();
        match url.split_once("?") {
            Some((_, raw_query_params)) => {
                let params = raw_query_params.split("&");
                for param in params {
                    match param.split_once("=") {
                        Some((key, value)) if key.len() > 0 && value.len() > 0 => {
                            query_params.insert(key.to_string(), value.to_string());
                        }
                        _ => (),
                    }
                }
            }
            None => (),
        }
        query_params
    }

    fn extract_params_from_url(url: &String, route: &Route) -> HashMap<String, String> {
        let (leading_path, _) = url.split_once("?").unwrap_or((url, ""));
        let mut params = HashMap::new();

        if let Some(captures) = route.path_regex.captures(leading_path) {
            for param_key in route.params.clone() {
                if let Some(capture) = captures.name(&*param_key) {
                    params.insert(param_key.to_string(), capture.as_str().to_string());
                }
            }
        }

        params
    }

    fn extract_body(http_string: &str) -> &str {
        let parts: Vec<&str> = http_string.split("\r\n\r\n").collect();
        if parts.len() > 1 {
            let body = parts[1]; // Der Body ist alles nach \r\n\r\n
            println!("Body: {}", body);
            body
        } else {
            println!("Kein Body gefunden.");
            ""
        }
    }

    pub fn listen(&self, port: u16, function: fn() -> ()) {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            // TODO: adding multithreading here
            let mut stream = stream.unwrap();
            // TODO: lernen wie man den buffer am besten setzt
            let mut buffer = [0; 1024]; // Buffer für die eingehende Anfrage

            // Lese die Anfrage aus dem Stream
            stream.read(&mut buffer).unwrap();

            // Konvertiere den Buffer zu einem String
            let utf8_request = String::from_utf8_lossy(&buffer);

            // Extrahiere die erste Zeile (enthält Methode, Route und HTTP-Version)
            let first_line = utf8_request.lines().next().unwrap_or(""); // TODO: ERROR Handling
            println!("first line: {}", first_line);

            let response = Response::default();

            // Zerlege die erste Zeile in Bestandteile
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            if parts.len() > 1 {
                let path = parts[1];
                println!("Path: {}", path);

                if let Some(route) = self.find_matching_route(path, parts[0]) {
                    let request = Request {
                        http_version: parts[2].to_string(),
                        headers: HashMap::new(),
                        params: Self::extract_params_from_url(&path.to_string(), route),
                        query_params: Self::extract_query_params_from_url(path),
                        body: Self::extract_body(&utf8_request).to_string(),
                    };

                    let mut response = Response::default();
                    let callback = route.callback;
                    callback(request, &mut response);

                    let raw_response = response.to_raw_http_response();
                    stream.write_all(raw_response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                } else {
                    /// 404 not found
                    let response = Response {
                        http_version: String::from("HTTP/1.1"),
                        http_status: HttpStatus::NotFound,
                        headers: HashMap::new(),
                        body: Self::extract_body(&utf8_request).to_string(),
                    };
                    let raw_response = response.to_raw_http_response();
                    stream.write_all(raw_response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
            }

            // Sende eine Antwort
            //let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
            // let raw_response = response.to_raw_http_response();
            // stream.write_all(raw_response.as_bytes()).unwrap();
            // stream.flush().unwrap();
        }

        function();
    }
}
