// #[macro_use]
// extern crate rocket;

fn main() {
    println!("Running server");

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(server::run_server());
}
