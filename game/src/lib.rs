use bevy_app::prelude::{App, Plugin, Update};
use bevy_ecs::{
    prelude::{in_state, IntoSystemConfigs, OnEnter, SystemSet},
    schedule::States,
};

use bevy_math::prelude::Vec2;

use bevy_rapier2d::prelude::{RapierConfiguration, RapierPhysicsPlugin};

pub use cricket_pong_base as base;

pub mod actions;

mod gameplay;

mod objects;
use gameplay::GamePhase;

mod ui;

// This is the plugin that attaches gameplay
// It allows a SystemSet type parameter so that different environments can attach
// the same logic and run them under the appropriate conditions.
pub struct GameplayPlugin<Set: SystemSet, State: States> {
    set: Set,
    startup_state: State,
}

impl<Set: SystemSet, State: States> GameplayPlugin<Set, State> {
    pub fn new(set: Set, startup_state: State) -> Self {
        GameplayPlugin { set, startup_state }
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
            app.add_plugins(GameplayMarkerPlugin)
                .add_state::<gameplay::GamePhase>()
                .insert_resource(RapierConfiguration {
                    gravity: Vec2::ZERO,
                    ..Default::default()
                })
                .add_plugins(RapierPhysicsPlugin::<()>::default());
        }

        // in all cases, add all the gameplay systems to the defined SystemSet
        app
            // TODO make this happen on a trigger
            .add_systems(
                OnEnter(self.startup_state),
                gameplay::spawn_scene.in_set(self.set),
            )
            .add_systems(
                OnEnter(GamePhase::Pitching),
                gameplay::ready_pitching_phase.in_set(self.set),
            )
            .add_systems(
                Update,
                (
                    gameplay::consume_actions,
                    gameplay::register_goals.run_if(in_state(GamePhase::Active)),
                )
                    .in_set(self.set),
            );
    }
}