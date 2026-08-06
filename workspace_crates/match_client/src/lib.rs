use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

use bevy::prelude::*;

use lightyear::netcode::Key;
use lightyear::prelude::client::*;
use lightyear::prelude::*;
pub fn connect_to_server(mut commands: Commands, address: String, token: String) {
    let client_id = 222;
    let address: SocketAddr = match address.parse() {
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
    commands.spawn((
        Client::default(),
        LocalAddr(client_addr),
        PeerAddr(address),
        Link::default(),
        ReplicationReceiver,
        NetcodeClient::new(auth, NetcodeConfig::default()),
        
    ));
}
