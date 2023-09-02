use std::{
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::prelude::{info, Commands, EventReader, NextState, Res, ResMut};

use bevy_replicon::{
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport, NetcodeTransportError},
        ConnectionConfig, RenetClient,
    },
    replication_core::NetworkChannels,
};

use crate::networking::ConnectionState;

pub(crate) fn inititate_connection(mut commands: Commands, network_channels: Res<NetworkChannels>) {
    let server_channels_config = network_channels.server_channels();
    let client_channels_config = network_channels.client_channels();

    let client = RenetClient::new(ConnectionConfig {
        server_channels_config,
        client_channels_config,
        ..Default::default()
    });

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let client_id = current_time.as_millis() as u64;
    let server_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5678);
    let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST.into(), 0))?;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: 0,
        server_addr,
        user_data: None,
    };
    let transport = NetcodeClientTransport::new(current_time, authentication, socket)?;

    commands.insert_resource(client);
    commands.insert_resource(transport);
}

// run with bevy_renet::on_client_connected
pub(crate) fn connection_events(mut state: ResMut<NextState<ConnectionState>>) {
    info!("Client connected to server");
    // enter the "InGame" state
    state.set(ConnectionState::InGame);
}

// run with bevy_renet::on_client_disconnected
pub(crate) fn disconnection_events(mut state: ResMut<NextState<ConnectionState>>) {
    info!("Client disconnected from server");
    // reset to loading state for now
    // eventually, we could use this to show an alert supporting user actions
    // or handle some sort of reconnection
    state.set(ConnectionState::Connecting);
}

pub fn rejection_events(mut transport_errors: EventReader<NetcodeTransportError>) {
    for error in transport_errors.iter() {
        info!("Client received a transport error: {}", error);
    }
}
