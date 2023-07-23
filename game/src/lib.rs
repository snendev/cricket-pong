use bevy_app::prelude::{App, Plugin, Update};
use bevy_ecs::{
    prelude::{in_state, OnEnter, States, SystemSet},
    schedule::{Condition, IntoSystemConfigs, OnExit},
};

use bevy_math::prelude::Vec2;

use bevy_rapier2d::prelude::{RapierConfiguration, RapierPhysicsPlugin};

pub use cricket_pong_base::{self as base, Over};

pub mod actions;
mod objects;
mod systems;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum GamePhase {
    #[default]
    Inactive,
    Preparing,
    Bowling,
    Active,
    GameOver,
}

// This is the plugin that attaches gameplay
// It allows a SystemSet type parameter so that different environments can attach
// the same logic and run them under the appropriate conditions.
pub struct GameplayPlugin<Set: SystemSet, State: States> {
    set: Set,
    active_screen: State,
}

impl<Set: SystemSet, State: States> GameplayPlugin<Set, State> {
    pub fn new(set: Set, active_screen: State) -> Self {
        GameplayPlugin { set, active_screen }
    }
}

// This marker plugin allows us to check that _some_ GameplayPlugin has been added
// Do this via GameplayPlugin::is_added
// This is useful so that we can assert this as a requirement for subsequent plugins,
// while also allowing the plugin to be generic over the SystemSet
pub struct GameplayMarkerPlugin;
impl Plugin for GameplayMarkerPlugin {
    fn build(&self, _app: &mut App) {}
}

impl GameplayMarkerPlugin {
    pub fn is_added(app: &App) -> bool {
        app.is_plugin_added::<GameplayMarkerPlugin>()
    }
}

impl<GameplaySet: SystemSet + Copy, State: States + Copy> Plugin
    for GameplayPlugin<GameplaySet, State>
{
    fn build(&self, app: &mut App) {
        // if this has not been added yet, initialize physics, the marker, and GamePhase state
        if !GameplayMarkerPlugin::is_added(app) {
            println!("Add state gamephase");
            app.add_plugins(GameplayMarkerPlugin)
                .add_state::<GamePhase>()
                .insert_resource(RapierConfiguration {
                    gravity: Vec2::ZERO,
                    ..Default::default()
                })
                .init_resource::<Over>()
                .add_plugins(RapierPhysicsPlugin::<()>::default());
        }

        // in all cases, add all the gameplay systems to the defined SystemSet
        app.add_systems(
            OnEnter(self.active_screen),
            systems::scene::spawn_scene.in_set(self.set),
        )
        .add_systems(
            OnExit(self.active_screen),
            (
                systems::scene::despawn_scene,
                systems::scene::deactivate_game_phase,
                systems::scene::cleanup_resources,
            )
                .in_set(self.set),
        )
        .add_systems(
            OnEnter(GamePhase::Preparing),
            systems::tick::ready_bowling_phase.in_set(self.set),
        )
        .add_systems(
            Update,
            (
                systems::tick::consume_actions
                    .run_if(in_state(GamePhase::Bowling).or_else(in_state(GamePhase::Active))),
                systems::scoring::register_goals.run_if(in_state(GamePhase::Active)),
            )
                .in_set(self.set),
        );
    }
}
