mod websockets;

pub async fn run_websockets() {
    println!("==================================");
    println!("==>> Welcome to WS server 0.0 <<==");
    println!("==================================");
    println!("\nStarting WS server");
    websockets::run_websockets().await;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
