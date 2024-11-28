use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct RexApp {
    routes: HashMap<String, fn(u16, u16) -> ()>,
}

impl RexApp {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn get(&mut self, path: String, function: fn(u16, u16) -> ()) {
        self.routes.insert(path, function);
    }

    pub fn post(&mut self, path: String, function: fn(u16, u16) -> ()) {
        self.routes.insert(path, function);
    }


    pub fn listen(&self, port: u16, function: fn() -> ()) {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 1024]; // Buffer für die eingehende Anfrage

            // Lese die Anfrage aus dem Stream
            stream.read(&mut buffer).unwrap();

            // Konvertiere den Buffer zu einem String
            let request = String::from_utf8_lossy(&buffer);

            // Extrahiere die erste Zeile (enthält Methode, Route und HTTP-Version)
            let first_line = request.lines().next().unwrap_or("");

            // Zerlege die erste Zeile in Bestandteile
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            if parts.len() > 1 {
                let route = parts[1]; // Der zweite Teil ist die Route
                println!("Route: {}", route);

                // Rufe die übergebene Funktion mit der Route auf
                //function(route.to_string());

                let routeFunction = self.routes.get(route);

                if routeFunction.is_some() {
                    routeFunction.unwrap()(1,1);
                }

            }

            // Sende eine Antwort
            let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }

        function();
    }
}
