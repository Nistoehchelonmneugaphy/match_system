use clap::Parser;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct UserId(pub String);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct MatchPlayerToken(pub String);
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct MatchFinishedCallbackToken(pub String);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MatchStartupConfig {
    match_finished_callback_token: MatchFinishedCallbackToken,
    match_finished_callback_url: String,
    players_tokens: Vec<(UserId, MatchPlayerToken)>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MatchFinishedMessage {
    match_finished_callback_token: MatchFinishedCallbackToken,
    statistics: MatchStatistics,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SecondsSinceEpoch(pub usize);
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MatchStatistics {
    player_points: Vec<(UserId, usize)>,
    match_time: SecondsSinceEpoch,
    buildings_built_count: Vec<(UserId, usize)>,
}
#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(long)]
    match_startup_data: String,
}

pub fn main() -> anyhow::Result<()> {
    let clap_args = Args::parse();
    let config: MatchStartupConfig = serde_json::from_str(&clap_args.match_startup_data)?;
    println!("Print {config:#?}");
    Ok(())
}
