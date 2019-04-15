#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate dotenv;

mod resources;
mod schema;
mod jwt;

use rocket_contrib::json::*;
use rocket::http::{ContentType, Status};
use rocket::response;
use rocket::request::Request;
use std::collections::HashMap;
use std::sync::Mutex;

#[macro_use] extern crate rocket_contrib;

#[database("main_db")]
struct MainDB(diesel::MysqlConnection);

#[get("/")]
fn index() -> &'static str {
    "API is live!"
}

#[derive(Debug)]
struct ResponseJson {
    data: JsonValue,
    status: Status,
}

struct InternalState {
    revoked_tokens: Mutex<HashMap<String, i64>>
}

impl<'r> response::Responder<'r> for ResponseJson {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        response::Response::build_from(self.data.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[catch(500)]
fn internal_error() -> ResponseJson {
    ResponseJson {
        data: json!({ "message": "Something went wrong!" }),
        status: Status::InternalServerError
    }
}

#[catch(401)]
fn unauthorized_error() -> ResponseJson {
    ResponseJson {
        data: json!({ "message": "Unauthorized" }),
        status: Status::Unauthorized
    }
}

fn main() {
    dotenv::dotenv().ok();

    rocket::ignite()
        .attach(MainDB::fairing())
        .manage(InternalState { revoked_tokens: Mutex::new(HashMap::new()) })
        .mount("/", routes![index])
        .mount("/user", resources::user::routes())
        .register(catchers![internal_error, unauthorized_error])
        .launch();
}
