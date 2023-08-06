use bevy::prelude::{
    in_state, App, Component, IntoSystemConfigs, IntoSystemSetConfig, IntoSystemSetConfigs,
    NextState, OnEnter, OnExit, Plugin, ResMut, States, SystemSet, Update,
};

use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, ReceiveEvents};

use cricket_pong_game::{
    base::{
        components::{
            batter::Batter,
            physics::{
                ExternalImpulse as SyncImpulse, Transform as SyncTransform,
                Velocity as SyncVelocity,
            },
        },
        protocol::protocol,
    },
    GameplayPlugin,
};

use crate::noop;

pub mod components;
pub(crate) mod resources;
use resources::TickHistory;

use self::rollback::rollback_component;

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
                ReceiveEvents.run_if(in_state(self.active_screen)),
                OnlineGameplaySet::Tick.run_if(in_state(self.active_screen)),
                OnlineGameplaySet::PrepareRollback.run_if(in_state(self.active_screen)),
                OnlineGameplaySet::Rollback.run_if(in_state(self.active_screen)),
            )
                .chain(),
        )
        .init_resource::<TickHistory>()
        .add_plugins(ClientPlugin::new(ClientConfig::default(), protocol()))
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
                events::receive_entity_assignment_message,
                events::handle_insert_position,
                events::spawn_predictions,
            )
                .run_if(in_state(self.active_screen))
                .in_set(ReceiveEvents),
        )
        .add_plugins(GameplayPlugin::new(
            OnlineGameplaySet::Tick,
            tick::send_and_prepare_inputs,
            noop,
        ))
        .add_systems(
            Update,
            (
                rollback_component::<SyncTransform>,
                rollback_component::<SyncVelocity>,
                rollback_component::<SyncImpulse>,
                rollback_component::<Batter>,
            )
                .in_set(OnlineGameplaySet::PrepareRollback),
        )
        .add_plugins(GameplayPlugin::new(
            OnlineGameplaySet::Rollback,
            rollback::replay_ticks,
            noop,
        ));
    }
}
