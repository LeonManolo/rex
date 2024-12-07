#[derive(Copy, Clone)]
pub enum HttpStatus {
    Ok = 200,
    Created = 201,
    NotFound = 404,
    InternalServerError = 500,
}

impl HttpStatus {
    pub fn text(&self) -> &str {
        match self {
            HttpStatus::Ok => "OK",
            HttpStatus::Created => "CREATED",
            HttpStatus::NotFound => "NOT FOUND",
            HttpStatus::InternalServerError => "INTERNAL SERVER ERROR",
            //TODO: some more
        }
    }
}
