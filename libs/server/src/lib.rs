#[macro_use]
extern crate rocket;

mod server;

use rocket::data::{Limits, ToByteUnit};
use rocket::{Build, Rocket};

/// Routing
fn rocket(port: u16) -> Rocket<Build> {
    // Check https://rocket.rs/v0.5-rc/guide/configuration/#custom-providers
    let figment = rocket::Config::figment()
        .merge(("port", port))
        .merge(("limits", Limits::new().limit("json", 256.mebibytes())));

    rocket::custom(figment)
        .mount("/", server::get_index_routes())
        .mount("/bc", server::get_blockchain_routes())
}

pub async fn run(port: u16) {
    if let Err(e) = rocket(port).launch().await {
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
