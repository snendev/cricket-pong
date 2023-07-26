use bevy::prelude::{App, Component, NextState, OnEnter, OnExit, Plugin, ResMut, States};

use crate::AppScreen;

pub(crate) mod components;
pub(crate) mod resources;

mod connection;
mod events;

mod rollback;
pub(crate) use rollback::receive_update_component_events;

mod tick;
pub(crate) use tick::send_and_prepare_inputs;

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
pub(crate) fn enter_online_game_state(mut state: ResMut<NextState<ConnectionState>>) {
    state.set(ConnectionState::Connecting);
}

// should run OnExit(AppScreen::OnlineGame)
pub(crate) fn exit_online_game_state(mut state: ResMut<NextState<ConnectionState>>) {
    state.set(ConnectionState::Disconnected);
}

struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppScreen::OnlineGame), enter_online_game_state)
            .add_systems(OnExit(AppScreen::OnlineGame), exit_online_game_state)
            // .add_systems()
            ;
    }
}
