use crate::http_status::HttpStatus;
use std::any::Any;
use std::collections::HashMap;

pub struct Response {
    http_version: String,
    http_status: HttpStatus,
    headers: HashMap<String, String>,
    body: String, // leerer string wenn kein body (json wird zu einem string konvertiert)
}

pub trait ToJson {
    fn to_json_string(&self) -> String;
}

impl Response {
    pub fn json_from_trait<T: ToJson>(&mut self, body: T) {
        self.body = body.to_json_string();
    }

    pub fn json_from_trait_array<T: ToJson>(&mut self, body: T) {
        self.body = body.to_json_string();
        self.headers
            .insert(String::from("content-type"), String::from("text/plain"));

        self.headers.insert(
            String::from("content-length"),
            self.body.len().to_string(),
        );
    }

    // type safety umgehen
    pub fn json_from_map<K, V>(&mut self, body: &HashMap<String, Box<dyn Any>>) {}

    pub fn send(&mut self, text: String) {
        self.headers
            .insert(String::from("content-type"), String::from("text/plain"));

        self.headers.insert(
            String::from("content-length"),
            text.len().to_string(),
        );
    }

    pub fn send_status(&mut self) {}

    pub fn set_header(&mut self, name: String, value: String) {
        // lowercase to prevent duplicate headers!
        self.headers.insert(name.to_lowercase(), value);
    }

    pub fn set_headers(&mut self, headers: HashMap<String, String>) {
        // iterate
    }
}
