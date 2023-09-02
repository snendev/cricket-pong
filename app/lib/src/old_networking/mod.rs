use bevy::prelude::{
    in_state, App, Component, IntoSystemConfigs, IntoSystemSetConfig, IntoSystemSetConfigs,
    NextState, OnEnter, OnExit, Plugin, PluginGroup, ResMut, States, SystemSet, Update,
};

use bevy_replicon::{server::ServerPlugin, ReplicationPlugins};
use cricket_pong_game::GameplayPlugin;

use crate::noop;

pub mod components;
pub(crate) mod resources;

mod connection;
mod events;
mod rollback;
mod tick;

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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, SystemSet)]
pub enum OnlineGameplaySet {
    Tick,
    PrepareRollback,
    Rollback,
}

pub struct OnlineGameplayPlugin<State: States> {
    active_screen: State,
}

impl<State> OnlineGameplayPlugin<State>
where
    State: States,
{
    pub fn new(active_screen: State) -> Self {
        OnlineGameplayPlugin { active_screen }
    }
}

impl<State> Plugin for OnlineGameplayPlugin<State>
where
    State: States + Copy,
{
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                OnlineGameplaySet::Tick.run_if(in_state(self.active_screen)),
                OnlineGameplaySet::PrepareRollback.run_if(in_state(self.active_screen)),
                OnlineGameplaySet::Rollback.run_if(in_state(self.active_screen)),
            )
                .chain(),
        )
        .add_plugins(ReplicationPlugins.build().disable::<ServerPlugin>())
        .add_state::<ConnectionState>()
        .add_systems(OnEnter(self.active_screen), enter_online_game_state)
        .add_systems(OnExit(self.active_screen), exit_online_game_state)
        .add_systems(
            OnEnter(ConnectionState::Connecting),
            connection::inititate_connection.run_if(in_state(self.active_screen)),
        )
        .add_systems(
            Update,
            (
                connection::connection_events,
                connection::disconnection_events,
                connection::rejection_events,
                // todo
                events::receive_entity_assignment_message,
                events::receive_score_message,
                events::handle_insert_position,
                events::spawn_predictions,
            )
                .run_if(in_state(self.active_screen)),
        )
        .add_plugins(GameplayPlugin::new(
            OnlineGameplaySet::Tick,
            tick::send_and_prepare_inputs,
            noop,
        ))
        // .add_systems(
        //     Update,
        //     (
        //         rollback_component::<SyncTranslation>,
        //         rollback_component::<SyncRotation>,
        //         rollback_component::<SyncVelocity>,
        //         rollback_component::<SyncImpulse>,
        //         rollback_component::<Batter>,
        //         rollback_component::<GamePhase>,
        //     )
        //         .in_set(OnlineGameplaySet::PrepareRollback),
        // )
        // .add_plugins(GameplayPlugin::new(
        //     OnlineGameplaySet::Rollback,
        //     rollback::replay_ticks,
        //     noop,
        // ))
        ;
    }
}
