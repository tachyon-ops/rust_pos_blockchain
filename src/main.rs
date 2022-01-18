#[macro_use]
extern crate rocket;

mod blockchain;
mod server;

#[get("/")]
fn index() -> String {
    format!("Hello World!")
}

#[launch]
fn run() -> _ {
    println!("Running server");
    rocket::build()
        .mount("/", routes![index])
        .mount("/bc", server::get_routes())
}
