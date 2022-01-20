use websocket::receiver::Reader;
use websocket::sync::{Client, Server, Writer};
use websocket::OwnedMessage;

use pos_blockchain::BlockChain;

type Sockets = (Reader<TcpStream>, Writer<TcpStream>);

lazy_static! {
    static ref CLIENTS: Mutex<HashMap<String, Sockets>> = Mutex::new(HashMap::new());
    static ref BLOCKCHAIN: Mutex<BlockChain> = Mutex::new(BlockChain::new());
}

pub fn listen(p2p_port: u16, peers: Vec<&str>) {
    let server = Server::bind(format!("127.0.0.1:{}", p2p_port)).unwrap();

    println!(
        "Listening for peer to peer connection on 127.0.0.1:{}",
        p2p_port
    );

    for request in server.filter_map(Result::ok) {
        // Spawn a new thread for each connection.
        thread::spawn(|| {
            println!("REQUEST");
            // if !request.protocols().contains(&"rust-websocket".to_string()) {
            //     request.reject().unwrap();
            //     return;
            // }

            let mut client = request.use_protocol("rust-websocket").accept().unwrap();
            let uuid = Uuid::new_v4().to_string();

            let ip = client.peer_addr().unwrap();

            println!("Connection from {}", ip);

            let message = OwnedMessage::Text("Hello".to_string());
            client.send_message(&message).unwrap();

            connect_socket(uuid.clone(), client);
        });
    }
    // to connect to the peers that we have specified
    // this.connectToPeers();
}

// after making connection to a socket
fn connect_socket(uuid: String, socket: Client<TcpStream>) {
    println!("Socket {} connected", uuid.clone());

    let mut clients = CLIENTS.lock().unwrap();
    // push the socket too the socket array
    let client = clients.insert(uuid.clone(), socket.split().unwrap());

    if let Some((mut r, mut s)) = client {
        for message in r.incoming_messages() {
            let message = message.unwrap();

            println!("Message {:?}", message.clone());

            match message {
                OwnedMessage::Close(_) => {
                    let message = OwnedMessage::Close(None);
                    s.send_message(&message).unwrap();
                    println!("Client {} disconnected", uuid.clone());
                    return;
                }
                OwnedMessage::Ping(ping) => {
                    let message = OwnedMessage::Pong(ping);
                    s.send_message(&message).unwrap();
                }
                _ => s.send_message(&message).unwrap(),
            }
        }
    }
}
