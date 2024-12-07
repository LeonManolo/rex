use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use regex::Regex;
use crate::request::Request;
use crate::response::Response;

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
    params: Vec<String>, // todo: delete?
}

impl RexApp {
    pub fn new() -> Self {
        Self {
            routes: vec![],
        }
    }

    pub fn get(&mut self, path: String, function: RouteCallback) {
        self.push_route("GET", path, function);
    }

    pub fn post(&mut self, path: String, function: RouteCallback) {
        self.push_route("POST", path, function);
    }



    fn push_route(&mut self,method: &str, path: String, function: RouteCallback) {

        // TODO: convert string to regex representation
        let regex = Regex::new(path.as_str()).unwrap();

        let mut params = vec![];
        for regex_group_name in regex.capture_names() {
            if let Some(group_name) = regex_group_name {
                params.push(group_name.to_string());
            }
        }

        let route = Route {
            method: method.to_string(),
            path: path.clone(), // TODO: clone entfernen
            // TODO: Zum testen ist der path einfach direkt ein regex
            path_regex: regex, // TODO: unsafe unwrap!
            callback: function,
            params: params,
        };
        self.routes.push(route);
    }

    fn find_matching_route(&self, path: &str, method: &str) -> Option<&Route> {
        for route in self.routes.iter() {
            if route.path_regex.is_match(path) && route.method == method {
                return Some(&route);
            }
        }

        None
    }

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




    pub fn listen(&self, port: u16, function: fn() -> ()) {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            // TODO: adding multithreading here
            let mut stream = stream.unwrap();
            let mut buffer = [0; 1024]; // Buffer für die eingehende Anfrage

            // Lese die Anfrage aus dem Stream
            stream.read(&mut buffer).unwrap();

            // Konvertiere den Buffer zu einem String
            let request = String::from_utf8_lossy(&buffer);

            // Extrahiere die erste Zeile (enthält Methode, Route und HTTP-Version)
            let first_line = request.lines().next().unwrap_or(""); // TODO: ERROR Handling
            println!("first line: {}", first_line);


            let response = Response::default();

            // Zerlege die erste Zeile in Bestandteile
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            if parts.len() > 1 {
                let path = parts[1];
                let route = path; // Der zweite Teil ist die Route
                println!("Route: {}", route);





                if let Some(route) = self.find_matching_route(route, parts[0]) {
                    let request = Request {
                        http_version: parts[2].to_string(),
                        headers: HashMap::new(),
                        params: HashMap::new(),
                        query_params: Self::extract_query_params_from_url(path),
                        body: String::new(),
                    };

                    let mut response = Response::default();
                    let callback = route.callback;
                    callback(request, &mut response);

                    let raw_response = response.to_raw_http_response();
                    stream.write_all(raw_response.as_bytes()).unwrap();
                    stream.flush().unwrap();

                } else {
                    // TODO: handle route not found 404
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
