mod rex_app;
mod response;
mod headers;
mod http_status;
mod http_method;

use rex_app::RexApp;

fn main() {
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
