use bevy::prelude::{
    in_state, system_adapter, App, Component, IntoSystem, IntoSystemConfigs, IntoSystemSetConfig,
    IntoSystemSetConfigs, NextState, OnEnter, OnExit, Plugin, PluginGroup, ResMut, States,
    SystemSet, Transform, Update,
};

use bevy_replicon::{server::ServerPlugin, ReplicationPlugins};

use cricket_pong_game::{
    base::{
        components::{ball::Ball, batter::Batter, fielder::Fielder},
        rapier::prelude::{ExternalImpulse, Velocity},
    },
    GameplayPlugin,
};
use network_base::{sync_from_replication, ReplicationStrategyPlugin};

mod events;
mod init;
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
        .add_plugins(ReplicationStrategyPlugin)
        .add_state::<ConnectionState>()
        .add_systems(OnEnter(self.active_screen), enter_online_game_state)
        .add_systems(OnExit(self.active_screen), exit_online_game_state)
        .add_systems(
            OnEnter(ConnectionState::Connecting),
            init::initialize_client
                .pipe(system_adapter::unwrap)
                .run_if(in_state(self.active_screen)),
        )
        .add_plugins(GameplayPlugin::new(
            OnlineGameplaySet::Tick,
            tick::send_and_prepare_inputs,
        ))
        .add_systems(
            Update,
            (
                sync_from_replication::<Transform>,
                sync_from_replication::<Velocity>,
                sync_from_replication::<ExternalImpulse>,
                sync_from_replication::<Fielder>,
                sync_from_replication::<Batter>,
                sync_from_replication::<Ball>,
                events::receive_entity_assignment_message, // TODO: entity assignment messages
            )
                .before(OnlineGameplaySet::Tick),
        );
    }
}
