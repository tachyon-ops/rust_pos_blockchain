#[macro_use]
extern crate rocket;

mod server;

use rocket::{Build, Rocket};

/// Routing
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", server::get_index_routes())
        .mount("/bc", server::get_blockchain_routes())
}

pub async fn run_server() {
    if let Err(e) = rocket().launch().await {
        println!("Whoops! Rocket didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
        drop(e);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
