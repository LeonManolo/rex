mod rex_app;
mod response;
mod headers;
mod http_status;
mod http_method;

use rex_app::RexApp;

use std::io::Write;
use std::net::TcpListener;
use regex::Regex;

struct RouteSegment {
    path: String,

}


fn main() {
    let re = Regex::new(r"^/users2/(?<otherId>[^/]+)$").unwrap();
    let url = String::from("/users/:id/abc");

    // let url_segments = url.split("/");

    // for url_segment in url_segments {
    //     println!("{}", url_segment);
    // }


    if let Some(captures) = re.captures(&url) {
        if let Some(id) = captures.name("id") {
            println!("{}", id.as_str());
        }
    }

    let mut app = RexApp::new();
    let port = 8080;

    app.get(r"^/users/(?<otherId>[^/]+)$".parse().unwrap(), |request, response| {
        // in der datenbank
        println!("HALLO VON USER")
    });

    app.get(r"^/users2/(?<otherId>[^/]+)$".parse().unwrap(), |request, response| {
        // in der datenbank
        println!("HALLO VON USERS2")

    });

    app.get("/users2/:idBla".parse().unwrap(), |request, response| {
        // in der datenbank
        println!("HALLO VON HEY")

    });

    app.listen(port, || {
        println!("Server started on Port: {}", 8080);
    })
}
