mod rex_app;

use rex_app::RexApp;

use std::io::Write;
use std::net::TcpListener;

fn main() {
    let mut app = RexApp::new();
    let port = 8080;

    app.get("/hallo-welt".parse().unwrap(), |request, response| {
        // in der datenbank
        println!("HALLO VON HALLO WELT")
    });

    app.get("/hey".parse().unwrap(), |request, response| {
        // in der datenbank
        println!("HALLO VON HEY")

    });

    app.listen(port, || {
        println!("Server started on Port: {}", 8080);
    })
}

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
//
//     for stream in listener.incoming() {
//         let mut stream = stream.unwrap();
//
//         // HTTP-Antwort mit Headern und Body
//         let body = "<h1>Hello, world!</h1>";
//         let response = format!(
//             "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
//             body.len(),
//             body
//         );
//
//         let response2 = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
//
//         stream.write_all(response2.as_bytes()).expect("Failed to write response");
//         stream.flush().unwrap();
//
//         println!("Connection established!");
//     }
// }
