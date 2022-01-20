use log::info;

// https://github.com/libp2p/rust-libp2p/blob/master/src/tutorial.rs
use futures::prelude::*;
use libp2p::multiaddr::multiaddr;
use libp2p::ping::{Ping, PingConfig};
use libp2p::swarm::{Swarm, SwarmEvent};
use libp2p::{identity, PeerId};
use std::error::Error;

async fn listen(port: u16, peers: Vec<&str>) -> Result<(), Box<dyn Error>> {
    // Setup logging
    env_logger::init();

    // Get address of this peer
    let my_addr = &format!("localhost:{}", &port);
    info!("Setting up {}", my_addr);

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    info!("Local peer id: {:?}", local_peer_id);

    let transport = libp2p::development_transport(local_key).await?;

    // Create a ping network behaviour.
    //
    // For illustrative purposes, the ping protocol is configured to
    // keep the connection alive, so a continuous sequence of pings
    // can be observed.
    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

    // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port.
    let my_addr = format!("/ip4/0.0.0.0/tcp/{}", port);
    swarm.listen_on(my_addr.parse()?)?;

    // Dial the peer identified by the multi-address given as the second
    // command-line argument, if any.

    for peer in peers {
        let splitted: Vec<&str> = peer.split(':').collect();
        let p: u16 = splitted[1].parse::<u16>().unwrap();
        let ip4: Vec<&str> = splitted[0].split('.').collect();
        let ip4_1: u8 = ip4[0].parse::<u8>().unwrap();
        let ip4_2: u8 = ip4[1].parse::<u8>().unwrap();
        let ip4_3: u8 = ip4[2].parse::<u8>().unwrap();
        let ip4_4: u8 = ip4[3].parse::<u8>().unwrap();
        let addr = multiaddr!(Ip4([ip4_1, ip4_2, ip4_3, ip4_4]), Tcp(p));
        swarm.dial(addr)?;
        println!("Dialed {}", peer)
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            SwarmEvent::Behaviour(event) => println!("{:?}", event),
            _ => {}
        }
    }

    Ok(())
}

pub async fn run(p2p_port: u16, peers: Vec<&str>) -> Result<(), Box<dyn Error>> {
    listen(p2p_port, peers).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
