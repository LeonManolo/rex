
#[derive(Copy, Clone)]
pub enum HttpStatus {
    Ok = 200,
    Created = 201,
    NotFound = 404,
}

impl HttpStatus {
    pub fn text(&self) -> String {
        match self {
            HttpStatus::Ok => {
                String::from("OK")
            }
            HttpStatus::Created => {
                String::from("CREATED")
            }
            HttpStatus::NotFound => {
                String::from("SOMETHING")
            }
        }
    }
}