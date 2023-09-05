use anyhow::Result;
use bevy_log::info;
use std::{
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy_ecs::prelude::{Commands, Res};

use bevy_replicon::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        ConnectionConfig, RenetServer,
    },
    replication_core::NetworkChannels,
};

pub(crate) fn initialize_server(
    mut commands: Commands,
    network_channels: Res<NetworkChannels>,
) -> Result<()> {
    let server_channels_config = network_channels.server_channels();
    let client_channels_config = network_channels.client_channels();

    let server = RenetServer::new(ConnectionConfig {
        server_channels_config,
        client_channels_config,
        ..Default::default()
    });

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let public_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5678);
    info!("Initializing server on {}", public_addr);
    let socket = UdpSocket::bind(public_addr)?;
    let server_config = ServerConfig {
        max_clients: 2,
        protocol_id: 0,
        public_addr,
        authentication: ServerAuthentication::Unsecure,
    };
    let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();

    commands.insert_resource(server);
    commands.insert_resource(transport);
    Ok(())
}
