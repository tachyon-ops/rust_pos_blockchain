use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

#[derive(Debug)]
pub struct ApiResponse {
    pub json: rocket_contrib::json::JsonValue,
    pub status: Status,
}

// impl<'r> Responder<'r, 'static> for Person {
//   fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
//       let person_string = format!("{}:{}", self.name, self.age);
//       Response::build()
//           .sized_body(person_string.len(), Cursor::new(person_string))
//           .raw_header("X-Person-Name", self.name)
//           .raw_header("X-Person-Age", self.age.to_string())
//           .header(ContentType::new("application", "x-person"))
//           .ok()
//   }
// }

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, _req: &Request) -> response::Result<'static> {
        let body = format!("{}", self.json.0);
        Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            // .sized_body(self.json.0.as_str(), std::io::Cursor::new(x))
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
