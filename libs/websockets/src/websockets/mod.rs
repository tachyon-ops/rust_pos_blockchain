// use std::{collections::HashMap, convert::Infallible, sync::Arc};
// use tokio::sync::{mpsc, RwLock};
// use warp::{ws::Message, Filter, Rejection};

// pub mod handlers;
// pub mod ws;

// #[derive(Debug, Clone)]
// pub struct Client {
//     pub client_id: String,
//     pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
// }

// type Clients = Arc<RwLock<HashMap<String, Client>>>;
// type Result<T> = std::result::Result<T, Rejection>;

pub async fn run(p2p_port: u16, peers: Vec<&str>) {
    let path = "ws";
    println!("==================================");
    println!("==>> Welcome to WS server 0.0 <<==");
    println!("==================================");
    println!("\nStarting WS server");
    println!("Server on ws://127.0.0.1:{}/{}", p2p_port, path);

    // let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    // println!("Configuring websocket route");
    // let ws_route = warp::path(path)
    //     .and(warp::ws())
    //     .and(with_clients(clients.clone()))
    //     .and_then(handlers::ws_handler);

    // let routes = ws_route.with(warp::cors().allow_any_origin());
    // warp::serve(routes).run(([127, 0, 0, 1], p2p_port)).await;
}

// fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
//     warp::any().map(move || clients.clone())
// }
