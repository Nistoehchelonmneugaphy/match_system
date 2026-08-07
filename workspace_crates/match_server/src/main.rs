use std::net::SocketAddr;

use anyhow::Ok;
use clap::Parser;
use game_server_structs::websocket_communication::MatchPlayer;
use match_server::{FromGameServer, ToGameServer, port_picker::pick_unused_port, websocket};
use tracing::{Level, debug, info};
use lightyear::{netcode::ConnectToken, prelude::*};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MatchStartupConfig {
    secret_token: String,
    game_server_communication_ws_url: String,
    players: Vec<MatchPlayer>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct MatchPlayerToken(pub String);
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct CallbackToken(pub String);

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(long)]
    match_startup_data: String,
    
    #[arg(long)]
    server_addr: SocketAddr,
    #[arg(long)]
    protocol_id: u64,

}


pub fn main() -> anyhow::Result<()> {
    set_log_level_to_info();
    let clap_args = Args::parse();
    let client_id_for_token = rand::random();
    let private_key = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    let token = ConnectToken::build(clap_args.server_addr, clap_args.protocol_id, client_id_for_token, private_key).expire_seconds(-1).timeout_seconds(-1).generate().expect("Could not generate token"); 
    use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
    let token_b = token.try_into_bytes().unwrap();
    let encoded_string = BASE64.encode(token_b);
    println!("Encoded token: {}", encoded_string);
    use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::io::Write;
  let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&token_b).unwrap();
    let compressed_bytes = encoder.finish().unwrap();
    let encoded_string_compressed = BASE64.encode(compressed_bytes);
    println!("compressed: {encoded_string_compressed}");

    //println!("Raw bytes :#? {:#?}", token_b);
    return Ok(());
    let config: MatchStartupConfig = serde_json::from_str(&clap_args.match_startup_data)?;
    info!("Match startup information: {config:#?}");
    let an_unused_port = pick_unused_port().expect("to get an unused port");
    info!("Found an unused port {an_unused_port}");
    // TODO: Establish websocket connection to the game server
    let (to_server_tx, to_server_rx) = tokio::sync::mpsc::unbounded_channel::<ToGameServer>();
    let (from_server_tx, from_server_rx) = tokio::sync::mpsc::unbounded_channel::<FromGameServer>();
    websocket::spawn_ws_task(
        config.game_server_communication_ws_url.clone(),
        config.secret_token.clone(),
        to_server_rx,
        from_server_tx,
    );
    Ok(())
}

fn set_log_level_to_info() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).ok();
}

