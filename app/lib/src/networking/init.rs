use anyhow::Result;
use std::{
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::prelude::{Commands, Res};

use bevy_replicon::{
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport},
        ConnectionConfig, RenetClient,
    },
    replication_core::NetworkChannels,
};

pub(crate) fn initialize_client(
    mut commands: Commands,
    network_channels: Res<NetworkChannels>,
) -> Result<()> {
    let server_channels_config = network_channels.server_channels();
    let client_channels_config = network_channels.client_channels();

    let client = RenetClient::new(ConnectionConfig {
        server_channels_config,
        client_channels_config,
        ..Default::default()
    });

    let current_time: std::time::Duration =
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let client_id = current_time.as_millis() as u64;
    let server_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5678);
    let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0))?;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: 0,
        server_addr,
        user_data: None,
    };
    let transport = NetcodeClientTransport::new(current_time, authentication, socket)?;

    commands.insert_resource(client);
    commands.insert_resource(transport);

    Ok(())
}
