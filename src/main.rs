fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    // rt.block_on(server::run_server());
    rt.block_on(websockets::run_websockets());
}
