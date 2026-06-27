pub mod port_picker;
pub mod websocket;


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToGameServer {
    content: ToGameServerContent,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ToGameServerContent {
    Ping,
    StartedAtPort { port: Port },
    MatchFinished { statistics: MatchStatistics },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FromGameServer {
    content: FromGameServerContent,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum FromGameServerContent {
    Ping,
    
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MatchStatistics {
    player_points: Vec<(UserId, usize)>,
    match_lasted_seconds: SecondsSinceEpoch,
    buildings_built_count: Vec<(UserId, usize)>,
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Port(pub u16);
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UserId(pub String);
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SecondsSinceEpoch(pub usize);