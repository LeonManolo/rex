use std::collections::HashMap;

/// HTTP request object
/// https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages#anatomy_of_an_http_message
pub struct HttpRequest {
    pub method: String,
    /// e.g. GET, POST, PUT, ...
    pub url: String,
    /// e.g. localhost:8080/something?hello=world
    pub url_query_params: HashMap<String, String>,
    /// e.g. ("hello", "world"),...
    pub version: String,
    /// e.g. HTTP/1.1
    pub headers: HashMap<String, String>,
    pub body: String, // leerer string wenn kein body (json wird zu einem string konvertiert)
}

impl HttpRequest {
    pub fn decode_from_buffer(buffer: [u8; 1024]) -> Option<HttpRequest> {
        let utf8_request = String::from_utf8_lossy(&buffer);

        /// HTTP start line:
        /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages#anatomy_of_an_http_message
        if let Some(start_line) = utf8_request.lines().next() {
            let start_line_parts: Vec<&str> = start_line.split_whitespace().collect();

            if let [method, url, http_version] = start_line_parts.as_slice() {
                return Some(HttpRequest {
                    method: method.to_string(),
                    url: url.to_string(),
                    url_query_params: Self::extract_query_params_from_url(url),
                    version: http_version.to_string(),
                    headers: HashMap::new(), // TODO: extract headers
                    body: Self::extract_body(&utf8_request).to_string(),
                });
            };
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

    fn extract_body(http_string: &str) -> &str {
        let parts: Vec<&str> = http_string.split("\r\n\r\n").collect();
        if let [_, body] = parts.as_slice() {
            body // Der Body ist alles nach \r\n\r\n
        } else {
            ""
        }
    }
}
