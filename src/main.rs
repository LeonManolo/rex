mod rex_app;
mod response;
mod headers;
mod http_status;
mod request;

use rex_app::RexApp;
use std::collections::HashMap;

use regex::Regex;
use std::io::Write;
use std::net::TcpListener;

struct RouteSegment {
    path: String,
}

fn extract_params_from_url(url: &String) -> HashMap<String, String> {
    let param_keys = vec!["otherId"];
    let re = Regex::new(r"^/users2/(?<otherId>[^/]+)$").unwrap();
    let mut params = HashMap::new();

    if let Some(captures) = re.captures(url) {
        for param_key in param_keys {
            if let Some(capture) = captures.name(param_key) {
                params.insert(param_key.to_string(), capture.as_str().to_string());
            }
        }
    }

    params
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

fn main() {

    println!(r"{}", prepare_path_regex(String::from("/users")));

    // for pair in extract_params_from_url(&String::from("/users2/123")) {
    //     println!("key: {}, value: {}", pair.0,pair.1);
    // }


    let mut app = RexApp::new();
    let port = 8080;

    app.get(r"^/users/(?<otherId>[^/]+)".parse().unwrap(), |request, response| {
        // in der datenbank
        println!("HALLO VON USER")
    });

    app.get(r"/users2/:idKeineAhnung".parse().unwrap(), |request, response| {
        // in der datenbank
        println!("HALLO VON USERS2")

    });

    app.get("/users2/:idBla".parse().unwrap(), |request, response| {
        // in der datenbank
        println!("HALLO VON HEY")

    });

    app.listen(port, || {
        //Hier kommen wir nie an!?
        println!("Server started on Port: {}", 8080);
    });
}
