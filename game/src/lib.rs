use std::marker::PhantomData;

use bevy_app::prelude::{App, Plugin, Update};
use bevy_ecs::prelude::{IntoSystem, IntoSystemConfigs, OnEnter, OnExit, States, SystemSet};
use bevy_math::prelude::Vec2;

use bevy_rapier2d::prelude::{RapierConfiguration, RapierPhysicsPlugin};

pub use cricket_pong_base::{self as base, actions::Actions, Over};

mod objects;
mod schedule;
mod systems;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum GamePhase {
    #[default]
    Inactive,
    Bowling,
    Active,
    GameOver,
}

// This is the plugin that attaches gameplay
// It allows a SystemSet type parameter so that different environments can attach
// the same logic and run them under the appropriate conditions.
pub struct GameplayPlugin<
    Set: SystemSet,
    State: States,
    T: IntoSystem<(), Vec<(u16, Actions)>, TM> + Copy,
    TM,
> {
    set: Set,
    active_screen: State,
    tick_system: T,
    tick_params_marker: PhantomData<TM>,
}

impl<Set, State, T, TM> GameplayPlugin<Set, State, T, TM>
where
    Set: SystemSet,
    State: States,
    T: IntoSystem<(), Vec<(u16, Actions)>, TM> + Copy,
{
    pub fn new(set: Set, active_screen: State, tick_system: T) -> Self {
        GameplayPlugin {
            set,
            active_screen,
            tick_system,
            tick_params_marker: PhantomData::<TM>,
        }
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

impl<GameplaySet, State, T, TM> Plugin for GameplayPlugin<GameplaySet, State, T, TM>
where
    GameplaySet: SystemSet + Copy,
    State: States + Copy,
    T: IntoSystem<(), Vec<(u16, Actions)>, TM> + Copy + Send + Sync + 'static,
    TM: Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        // if this has not been added yet, initialize physics, the marker, and GamePhase state
        if !GameplayMarkerPlugin::is_added(app) {
            let (schedule_label, schedule) = schedule::build_core_tick_schedule();
            app.add_plugins(GameplayMarkerPlugin)
                .add_state::<GamePhase>()
                .init_resource::<Over>()
                .init_resource::<Actions>()
                .insert_resource(RapierConfiguration {
                    gravity: Vec2::ZERO,
                    // TODO: configure timestep_mode during rollback
                    ..Default::default()
                })
                .add_plugins(RapierPhysicsPlugin::<()>::default().with_default_system_setup(false))
                .add_schedule(schedule_label, schedule);
        }

        // in all cases, add all the gameplay systems to the defined SystemSet
        app.add_systems(
            Update,
            (
                (
                    systems::scene::attach_ball_physics_components,
                    systems::scene::attach_fielder_physics_components,
                    systems::scene::attach_batter_physics_components,
                    systems::scene::attach_boundary_physics_components,
                    systems::scene::attach_wicket_physics_components,
                ),
                self.tick_system.pipe(schedule::run_core_game_loop),
            )
                .chain()
                .in_set(self.set),
        )
        .add_systems(
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
        );
    }
}
