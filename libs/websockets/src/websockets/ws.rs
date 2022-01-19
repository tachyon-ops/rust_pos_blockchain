use super::{Client, Clients};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;

use warp::ws::{Message, WebSocket};

pub async fn client_connection(ws: WebSocket, clients: Clients) {
    println!("establishing client connection... {:?}", ws);
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));
    let uuid = Uuid::new_v4().to_simple().to_string();
    let new_client = Client {
        client_id: uuid.clone(),
        sender: Some(client_sender),
    };
    println!("{} client added", new_client.client_id);
    clients.write().await.insert(uuid.clone(), new_client);

    send_welcome_message(&clients, &uuid.clone()).await;

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                println!("error receiving message for id {}): {}", uuid.clone(), e);
                break;
            }
        };
        client_msg(&uuid, msg, &clients).await;
    }
    clients.write().await.remove(&uuid);
    println!("{} disconnected", uuid);
}

async fn send_welcome_message(clients: &Clients, uuid: &str) {
    let this_clients = clients.read().await;
    let some_client = this_clients
        .iter()
        .find(|(_, client)| client.client_id == uuid);
    if let Some((_, client)) = some_client {
        if let Some(sender) = &client.sender {
            let _ = sender.send(Ok(Message::text(format!(
                "You are client ID: {}",
                uuid.clone()
            ))));
        }
    }
}

async fn client_msg(client_id: &str, msg: Message, clients: &Clients) {
    println!("received message from {}: {:?}", client_id, msg);
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };
    if message == "ping" || message == "ping\n" {
        match clients.read().await.get(client_id) {
            Some(v) => {
                if let Some(sender) = &v.sender {
                    println!("sending pong");
                    let _ = sender.send(Ok(Message::text("pong")));
                }
            }
            None => return,
        }
        return;
    } else {
        // send to all others
        clients
            .read()
            .await
            .iter()
            .filter(|(_, client)| client.client_id != client_id)
            .for_each(|(client_id, client)| {
                if let Some(sender) = &client.sender {
                    println!("Sending msg [{}] to client ID {}", message, client_id);
                    let _ = sender.send(Ok(Message::text(message.clone())));
                }
            })
    }
}
