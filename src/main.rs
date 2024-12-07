mod headers;
mod http_request_decoder;
mod http_status;
mod request;
mod response;
mod rex_app;

use crate::request::FromJson;
use crate::response::ToJson;
use rex_app::RexApp;
use std::io::Write;

/// Example response json
struct MyCustomJsonRequestBodyDto {
    my_prop: String,
    my_other_prop: String,
}
impl FromJson for MyCustomJsonRequestBodyDto {
    fn from_json_string(json_string: String) -> Self {
        // custom conversion logic here (more complex)
        MyCustomJsonRequestBodyDto {
            my_prop: "".to_string(),
            my_other_prop: "".to_string(),
        }
    }
}

/// Example request json
struct MyCustomJsonResponseBodyDto {
    test_prop: String,
    some_number: i32,
}

impl ToJson for MyCustomJsonResponseBodyDto {
    fn to_json_string(&self) -> String {
        // custom conversion logic
        format!(
            r#"{{"test_prop":"{}","some_number":{}}}"#,
            self.test_prop, self.some_number,
        )
    }
}

fn main() {
    let mut app = RexApp::new();
    let port = 8080;

    app.get("/users/:id", |request, response| {
        // in der datenbank
        println!("HALLO VON USER");
        println!("body: {}", request.body);

        // 127.0.0.1:8080/users/111?myparam=hallo&myotherparam=3
        // query params can be any name! Query params values are always a string and have to be converted to other types e.g. integer,...
        let my_custom_query_param = request.query_params.get("myparam");
        let my_custom_query_param2 = request.query_params.get("myotherparam");

        let id = request.param("id");

        response.set_header("my-custom-header", "custom-header-value");

        let response_text = format!(
            "My awesome body! myparam: {} and myotherparam: {} and my param path param id is = {}",
            my_custom_query_param.unwrap_or(&String::new()),
            my_custom_query_param2.unwrap_or(&String::new()),
            id.unwrap_or(String::new()),
        );

        return response.send_text(response_text.as_str());

        // or use custom json helper method
        let response_json = MyCustomJsonResponseBodyDto {
            test_prop: "THIS IS MY CUSTOM PROP".to_string(),
            some_number: 42,
        };
        return response.send_json_from_trait(response_json);
    });

    app.get("/users2/:idKeineAhnung", |request, response| {
        // in der datenbank
        println!("HALLO VON USERS2");
        return response.send_text("HALLO WELT VON /users2 mit");
    });

    app.get("/users/:id/hallo", |request, response| {
        // in der datenbank
        println!("HALLO VON :id Hallo");
    });

    app.listen(port, || {
        //Hier kommen wir nie an!?
        println!("Server started on Port: {}", 8080);
    });
}
