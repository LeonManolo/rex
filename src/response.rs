use crate::http_status::HttpStatus;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::format;

pub struct Response {
    pub(crate) http_version: String,
    pub(crate) http_status: HttpStatus,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: String, // leerer string wenn kein body (json wird zu einem string konvertiert)
}

pub trait ToJson {
    fn to_json_string(&self) -> String;
}

impl Response {
    pub fn default() -> Self {
        Self {
            http_version: String::from("HTTP/1.1"),
            http_status: HttpStatus::Ok,
            headers: HashMap::from([(String::from("content-length"), String::from("0"))]),
            body: String::new(),
        }
    }

    fn headers_to_string(&self) -> String {
        let mut headers_as_string = String::new();

        for header in self.headers.iter() {
            headers_as_string.push_str(&format!("{}: {}\r\n", header.0, header.1));
        }

        headers_as_string
    }

    pub fn to_raw_http_response(&self) -> String {
        let http_status_code = self.http_status as u16;
        let http_response_string = format!(
            "{} {} {}\r\n{}\r\n{}",
            self.http_version,
            http_status_code.to_string(),
            self.http_status.text(),
            self.headers_to_string(),
            self.body,
        );

        //println!("{}", http_response_string);
        http_response_string
        // "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n".to_string()
    }

    /// Serializes the given object to JSON and sets it as the HTTP response body.
    /// Also updates the `Content-Type` and `Content-Length` headers.
    ///
    /// # Parameters
    /// - `body`: An object implementing the `ToJson` trait, to be serialized into JSON.
    ///
    pub fn send_json_from_trait<T: ToJson>(&mut self, body: T) {
        self.body = body.to_json_string();
        self.headers
            .insert(String::from("content-type"), String::from("application/json"));

        self.headers
            .insert(String::from("content-length"), self.body.len().to_string());
    }

    // pub fn send_json_from_trait_array<T: ToJson>(&mut self, body: T) {
    //     self.body = body.to_json_string();
    //     // for loop here
    //     self.headers
    //         .insert(String::from("content-type"), String::from("text/plain"));
    //
    //     self.headers
    //         .insert(String::from("content-length"), self.body.len().to_string());
    // }

    // type safety umgehen
    pub fn json_from_map<K, V>(&mut self, body: &HashMap<String, Box<dyn Any>>) {}

    pub fn send(&mut self, text: &str) {
        self.body = text.to_string();
        self.headers
            .insert(String::from("content-type"), String::from("text/plain"));

        self.headers
            .insert(String::from("content-length"), text.len().to_string());
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
