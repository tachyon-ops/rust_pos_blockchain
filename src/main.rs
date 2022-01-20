use clap::Parser;

/// BlockChain PoS completely written in RUST
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Do you wish to run the server? If not it will run websockets.
    #[clap(short, long)]
    server: bool,

    /// Server/Websocket PORT
    #[clap(short, long, default_value_t = 8000)]
    port: u16,

    /// Comma separated list of Peers
    #[clap(long, default_value = "")]
    peers: String,
}

fn main() {
    let args = Args::parse();

    let rt = tokio::runtime::Runtime::new().unwrap();

    let peers_string = args.peers;
    let peers: Vec<&str>;
    if peers_string.len() > 0 {
        peers = peers_string.split(",").collect();
    } else {
        peers = vec![];
    }
    println!("Peers: {:?}", peers);

    // if args.server {
    //     rt.block_on(server::run(args.port));
    // } else {
    //     rt.block_on(websockets::run(args.port, peers));
    // }
    rt.block_on(p2p_server::run(args.port, peers));
}
