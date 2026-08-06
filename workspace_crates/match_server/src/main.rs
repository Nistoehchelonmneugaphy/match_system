use clap::Parser;
use game_server_structs::websocket_communication::MatchPlayer;
use match_server::{FromGameServer, ToGameServer, port_picker::pick_unused_port, websocket};
use tracing::{Level, info};

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
}


pub fn main() -> anyhow::Result<()> {
    set_log_level_to_info();
    let clap_args = Args::parse();
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
