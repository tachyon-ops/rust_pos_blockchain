use log::info;

use async_std::{io, task};
use futures::{
    prelude::{stream::StreamExt, *},
    select,
};
use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    identity,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    multiaddr::multiaddr,
    swarm::SwarmEvent,
    NetworkBehaviour, PeerId, Swarm,
};
use std::error::Error;

// https://github.com/libp2p/rust-libp2p/blob/master/src/tutorial.rs

async fn listen(port: u16, peers: Vec<&str>) -> Result<(), Box<dyn Error>> {
    // Setup logging
    env_logger::init();

    // Get address of this peer
    let my_addr = &format!("localhost:{}", &port);
    println!("Setting up {}", my_addr);

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    let transport = libp2p::development_transport(local_key).await?;

    // Create a Floodsub topic
    let floodsub_topic = floodsub::Topic::new("chat");

    // We create a custom network behaviour that combines floodsub and mDNS.
    // In the future, we want to improve libp2p to make this easier to do.
    // Use the derive to generate delegating NetworkBehaviour impl and require the
    // NetworkBehaviourEventProcess implementations below.
    #[derive(NetworkBehaviour)]
    #[behaviour(out_event = "OutEvent")]
    struct MyBehaviour {
        floodsub: Floodsub,
        mdns: Mdns,

        // Struct fields which do not implement NetworkBehaviour need to be ignored
        #[behaviour(ignore)]
        #[allow(dead_code)]
        ignored_member: bool,
    }

    #[derive(Debug)]
    enum OutEvent {
        Floodsub(FloodsubEvent),
        Mdns(MdnsEvent),
    }

    impl From<MdnsEvent> for OutEvent {
        fn from(v: MdnsEvent) -> Self {
            Self::Mdns(v)
        }
    }

    impl From<FloodsubEvent> for OutEvent {
        fn from(v: FloodsubEvent) -> Self {
            Self::Floodsub(v)
        }
    }

    // Create a ping network behaviour.
    //
    // For illustrative purposes, the ping protocol is configured to
    // keep the connection alive, so a continuous sequence of pings
    // can be observed.
    // let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    let mut swarm = {
        let mdns = task::block_on(Mdns::new(MdnsConfig::default()))?;
        let mut behaviour = MyBehaviour {
            floodsub: Floodsub::new(local_peer_id),
            mdns,
            ignored_member: false,
        };

        behaviour.floodsub.subscribe(floodsub_topic.clone());
        Swarm::new(transport, behaviour, local_peer_id)
    };

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

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port.
    let my_addr = format!("/ip4/0.0.0.0/tcp/{}", port);
    swarm.listen_on(my_addr.parse()?)?;

    loop {
        select! {
            line = stdin.select_next_some() => swarm
                .behaviour_mut()
                .floodsub
                .publish(floodsub_topic.clone(), line.expect("Stdin not to close").as_bytes()),
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {:?}", address);
                }
                SwarmEvent::Behaviour(OutEvent::Floodsub(
                    FloodsubEvent::Message(message)
                )) => {
                    println!(
                        "Received: '{:?}' from {:?}",
                        String::from_utf8_lossy(&message.data),
                        message.source
                    );
                }
                SwarmEvent::Behaviour(OutEvent::Mdns(
                    MdnsEvent::Discovered(list)
                )) => {
                    for (peer, _) in list {
                        swarm
                            .behaviour_mut()
                            .floodsub
                            .add_node_to_partial_view(peer);
                    }
                }
                SwarmEvent::Behaviour(OutEvent::Mdns(MdnsEvent::Expired(
                    list
                ))) => {
                    for (peer, _) in list {
                        if !swarm.behaviour_mut().mdns.has_node(&peer) {
                            swarm
                                .behaviour_mut()
                                .floodsub
                                .remove_node_from_partial_view(&peer);
                        }
                    }
                },
                _ => {}
            }
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
