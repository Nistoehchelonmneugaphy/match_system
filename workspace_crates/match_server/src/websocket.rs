use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use http::HeaderValue;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream,
    tungstenite::{self, Message, client::IntoClientRequest},
};
use tracing::{trace, warn};

use crate::{FromGameServer, ToGameServer};


pub enum BreakOrContinue {
    Break,
    Continue,
}

pub fn spawn_ws_task(
    url: String,
    cookie: String,
    to_server_rx: tokio::sync::mpsc::UnboundedReceiver<ToGameServer>,
    from_server_tx: tokio::sync::mpsc::UnboundedSender<FromGameServer>,
) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to start Tokio runtime");
        rt.block_on(ws_loop(url, cookie, to_server_rx, from_server_tx));
    });
}

async fn ws_loop(
    url: String,
    cookie: String,
    mut to_server_rx: tokio::sync::mpsc::UnboundedReceiver<ToGameServer>,
    from_server_tx: tokio::sync::mpsc::UnboundedSender<FromGameServer>,
) {
    let mut request = match url.clone().into_client_request() {
        Ok(r) => r,
        Err(e) => {
            warn!("Invalid WS URL {url}: {e}");
            return;
        }
    };
    let cookie_value = match HeaderValue::from_str(&cookie) {
        Ok(v) => v,
        Err(e) => {
            warn!("Invalid cookie header value: {e}");
            return;
        }
    };
    request.headers_mut().insert("cookie", cookie_value);

    let (ws_stream, _) = match tokio_tungstenite::connect_async(request).await {
        Ok(pair) => pair,
        Err(e) => {
            warn!("Failed to connect to ws server: {e}");
            return;
        }
    };
    let (mut ws_writer, mut ws_reader) = ws_stream.split();

    loop {
        tokio::select! {
            msg = ws_reader.next() => {
                let ws_msg = to_ws_message(msg);
                match handle_incoming(&from_server_tx, ws_msg) {
                    BreakOrContinue::Break => break,
                    BreakOrContinue::Continue => {}
                }
            }
            Some(msg) = to_server_rx.recv() => {
                match send_one(&mut ws_writer, &msg).await {
                    BreakOrContinue::Break => break,
                    BreakOrContinue::Continue => {}
                }
            }
        }
    }
}

fn to_ws_message(
    msg: Option<Result<tungstenite::protocol::Message, tungstenite::error::Error>>,
) -> WsMessage {
    match msg {
        Some(Ok(Message::Text(text))) => WsMessage::Text(text.to_string()),
        Some(Ok(Message::Close(_))) | None => WsMessage::Close,
        Some(Ok(_)) => WsMessage::Other,
        Some(Err(e)) => {
            warn!("WS read error: {e}");
            WsMessage::Close
        }
    }
}

async fn send_one(
    ws_writer: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    msg: &ToGameServer,
) -> BreakOrContinue {
    let Some(text) = serialize_outgoing(msg) else {
        return BreakOrContinue::Continue;
    };
    match ws_writer.send(Message::Text(text.into())).await {
        Ok(()) => BreakOrContinue::Continue,
        Err(e) => {
            warn!("WS write failed: {e}");
            BreakOrContinue::Break
        }
    }
}

/// Platform-agnostic view of a WS message, so parsing logic is written once.
pub(crate) enum WsMessage {
    Text(String),
    Close,
    Other, // binary/ping/pong/etc — ignored uniformly
}

/// Shared serialization logic for outgoing messages.
pub(crate) fn serialize_outgoing(msg: &ToGameServer) -> Option<String> {
    match serde_json::to_string(msg) {
        Ok(text) => Some(text),
        Err(e) => {
            warn!("Failed to serialize ToGameServer: {e}");
            None
        }
    }
}

/// Shared parsing logic — used by both native and wasm backends.
pub(crate) fn handle_incoming(
    from_server_tx: &tokio::sync::mpsc::UnboundedSender<FromGameServer>,
    msg: WsMessage,
) -> BreakOrContinue {
    match msg {
        WsMessage::Text(text) => match serde_json::from_str::<FromGameServer>(&text) {
            Ok(parsed) => {
                if from_server_tx.send(parsed).is_err() {
                    warn!("from_server channel closed, receiver dropped");
                    return BreakOrContinue::Break;
                }
            }
            Err(e) => warn!("Failed to parse ServerMessage: {e}, raw: {text}"),
        },
        WsMessage::Close => {
            warn!("Server closed connection");
            return BreakOrContinue::Break;
        }
        WsMessage::Other => {
            trace!("Ignoring non-text WS message");
        }
    }
    BreakOrContinue::Continue
}
