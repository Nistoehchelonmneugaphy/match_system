use clap::Parser;
use server::port_picker::pick_unused_port;
use tracing::{Level, info};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MatchStartupConfig {
    callback_token: CallbackToken,
    match_port_callback_url: String,
    match_finished_callback_url: String,
    players_tokens: Vec<(UserId, MatchPlayerToken)>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MatchFinishedMessage {
    callback_token: CallbackToken,
    statistics: MatchStatistics,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MatchStatistics {
    player_points: Vec<(UserId, usize)>,
    match_lasted_seconds: SecondsSinceEpoch,
    buildings_built_count: Vec<(UserId, usize)>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct UserId(pub String);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct MatchPlayerToken(pub String);
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct CallbackToken(pub String);
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SecondsSinceEpoch(pub usize);

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(long)]
    match_startup_data: String,
}

struct Port(pub u16);
pub fn main() -> anyhow::Result<()> {
    set_log_level_to_info();
    let clap_args = Args::parse();
    let config: MatchStartupConfig = serde_json::from_str(&clap_args.match_startup_data)?;
    info!("Match startup information: {config:#?}");
    let an_unused_port = pick_unused_port().expect("to get an unused port");
    info!("Found an unused port {an_unused_port}");
    // TODO: Establish websocket connection to the game server
    Ok(())
}


fn set_log_level_to_info() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).ok();
}
