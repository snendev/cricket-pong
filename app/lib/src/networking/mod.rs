use bevy::prelude::{
    in_state, App, Component, IntoSystemConfigs, NextState, OnEnter, OnExit, Plugin, ResMut,
    States, Update,
};

use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin};

use cricket_pong_game::base::protocol::protocol;

use crate::AppScreen;

pub(crate) mod components;
pub(crate) mod resources;

mod connection;
mod events;

mod rollback;
pub(crate) use rollback::receive_update_component_events;

mod tick;
pub(crate) use tick::send_and_prepare_inputs;

use self::resources::TickHistory;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
pub(crate) enum ConnectionState {
    #[default]
    Disconnected,
    Connecting,
    InGame,
}

#[derive(Component)]
pub(crate) struct MyPlayer;

// should run OnEnter(AppScreen::OnlineGame)
fn enter_online_game_state(mut state: ResMut<NextState<ConnectionState>>) {
    state.set(ConnectionState::Connecting);
}

// should run OnExit(AppScreen::OnlineGame)
fn exit_online_game_state(mut state: ResMut<NextState<ConnectionState>>) {
    state.set(ConnectionState::Disconnected);
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TickHistory>()
            .add_plugins(ClientPlugin::new(ClientConfig::default(), protocol()))
            .add_state::<ConnectionState>()
            .add_systems(OnEnter(AppScreen::OnlineGame), enter_online_game_state)
            .add_systems(OnExit(AppScreen::OnlineGame), exit_online_game_state)
            .add_systems(
                OnEnter(ConnectionState::Connecting),
                connection::inititate_connection.run_if(in_state(AppScreen::OnlineGame)),
            )
            .add_systems(
                Update,
                (
                    connection::connection_events,
                    connection::disconnection_events,
                    connection::rejection_events,
                    events::receive_entity_assignment_message,
                    events::receive_insert_component_events,
                )
                    .run_if(in_state(AppScreen::OnlineGame)),
            );
    }
}
