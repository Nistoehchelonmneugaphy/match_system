use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

use bevy::prelude::*;

use lightyear::netcode::Key;
use lightyear::prelude::client::*;
use lightyear::prelude::*;
pub fn connect_to_server(mut commands: Commands, address_string: String, token: String) {
    let client_id = 222;
    let address: SocketAddr = match address_string.parse() {
        Ok(a) => a,
        Err(e) => {
            debug!("Parse error: {e}");
            return;
        }
    };

    let auth = Authentication::Manual {
        server_addr: address,
        client_id,
        private_key: Key::default(),
        protocol_id: 0,
    };
    let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 32000);
    let nc_client = match NetcodeClient::new(auth, NetcodeConfig::default()) {
        Ok(c) => c,
        Err(e) => {
            debug!("NC Client new error: {e}");
            return;
        }
    };
   let client_entity =  commands.spawn((
        Client::default(),
        LocalAddr(client_addr),
        PeerAddr(address),
        Link::default(),
        ReplicationReceiver,
        nc_client,
        WebSocketClientIo::from_url(ClientConfig, format!("ws://{address_string}"))
    )).id();
    commands.trigger(Connect {entity: client_entity});
}
