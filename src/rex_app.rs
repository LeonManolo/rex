use std::io::{Read, Write};
use std::net::TcpListener;
use regex::Regex;
use crate::response::Response;

pub struct RexApp {
    routes: Vec<Route>,
}

type RouteCallback = fn(request: u16,response: u16) -> ();

struct Route {
    method: String,
    path: String,
    path_regex: Regex,
    callback: RouteCallback,
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
        let path = RexApp::prepare_path_regex(path);

        let route = Route {
            method: method.to_string(),
            path: path.clone(), // TODO: clone entfernen
            // TODO: Zum testen ist der path einfach direkt ein regex
            path_regex: Regex::new(path.as_str()).unwrap(), // TODO: unsafe unwrap!
            callback: function,
        };
        self.routes.push(route);
    }

    fn prepare_path_regex(path: String) -> String {
        let mut regrex_string = String::from("^");

        let url_segments = path.split("/");

        for url_segment in url_segments {
            if url_segment.contains(":") {
                let value = url_segment.trim_start_matches(':');
                regrex_string.push_str(&format!("/(?<{}>[^/]+)", value));
            } else if !url_segment.is_empty() {
                regrex_string.push_str(&format!("/{}", url_segment));
            }
        }

        regrex_string.push_str("$");
        regrex_string
    }

    fn find_matching_route(&self, path: &str, method: &str) -> Option<&Route> {
        for route in self.routes.iter() {
            if route.path_regex.is_match(path) && route.method == method {
                return Some(&route);
            }
        }

        None
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
                let route = parts[1]; // Der zweite Teil ist die Route
                println!("Route: {}", route);





                if let Some(route) = self.find_matching_route(route, parts[0]) {
                    let callback = route.callback;
                    callback(1,2);

                } else {
                    // TODO: fehler
                }


            }

            // Sende eine Antwort
            //let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
            let raw_response = response.to_raw_http_response();
            stream.write_all(raw_response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }

        function();
    }
}
