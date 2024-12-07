use crate::http_status::HttpStatus;
use std::any::Any;
use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub url: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub params: Vec<(String, String)>, // Todo: dyn Any ?
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

    pub fn param(&self, key: &str) -> Option<String> {
        let param_pair = self.params.iter().find(|(k, _)| k.eq(key));

        match param_pair {
            Some((_, value)) => Some(value.clone()),
            None => None,
        }
    }

    pub fn query_param() {} // Todo implementieren wie "param" funktion

    pub fn json_body<T: FromJson>(&self) -> T {
        T::from_json_string(self.body.clone())
    }
}
