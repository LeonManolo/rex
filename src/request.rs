use std::any::Any;
use std::collections::HashMap;
use crate::http_status::HttpStatus;

pub struct Request {
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>, // Todo: dyn Any ?
    pub query_params: HashMap<String, String>,
    pub body: String, // leerer string wenn kein body (json wird zu einem string konvertiert)
}

pub trait FromJson {
    fn from_json_string(json_string: String) -> Self;
}

impl Request {
    pub fn header(&self, key: String) -> Option<String> {
        self.headers.get(&key).cloned()
    }

    pub fn param(&self, key: String) -> Option<String> {
        self.params.get(&key).cloned()
    }

    pub fn json_body<T: FromJson>(&self) -> T {
        T::from_json_string(self.body.clone())
    }
}