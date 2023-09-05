use std::marker::PhantomData;

use bevy_app::prelude::{App, Plugin, Update};
use bevy_ecs::prelude::{IntoSystem, IntoSystemConfigs, SystemSet};
use bevy_math::prelude::Vec2;

pub use cricket_pong_base::{
    self as base,
    actions::Actions,
    components::instance::GameInstance,
    rapier::prelude::{RapierConfiguration, RapierPhysicsPlugin, TimestepMode},
};

mod objects;
mod schedule;
mod systems;

// This is the plugin that attaches gameplay
// It allows a SystemSet type parameter so that different environments can attach
// the same logic and run them under the appropriate conditions.
pub struct GameplayPlugin<Set, T, TM>
where
    Set: SystemSet,
    T: IntoSystem<(), Vec<(u16, Actions)>, TM> + Copy,
{
    tick_set: Set,
    tick_system: T,
    tick_params_marker: PhantomData<TM>,
}

impl<Set, T, TM> GameplayPlugin<Set, T, TM>
where
    Set: SystemSet,
    T: IntoSystem<(), Vec<(u16, Actions)>, TM> + Copy,
{
    pub fn new(tick_set: Set, tick_system: T) -> Self {
        GameplayPlugin {
            tick_set,
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

impl<Set, T, TM> Plugin for GameplayPlugin<Set, T, TM>
where
    Set: SystemSet + Copy,
    T: IntoSystem<(), Vec<(u16, Actions)>, TM> + Copy + Send + Sync + 'static,
    TM: Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        // if this has not been added yet, initialize physics, the marker, and GamePhase state
        if !GameplayMarkerPlugin::is_added(app) {
            let (schedule_label, schedule) = schedule::build_core_tick_schedule();
            app.add_plugins(GameplayMarkerPlugin)
                .add_event::<systems::scoring::ScoreEvent>()
                .init_resource::<Actions>()
                .insert_resource(RapierConfiguration {
                    gravity: Vec2::ZERO,
                    // TODO: improve timestep syncing
                    timestep_mode: TimestepMode::Fixed {
                        dt: 1. / 60.,
                        substeps: 1,
                    },
                    ..Default::default()
                })
                .add_plugins(
                    RapierPhysicsPlugin::<schedule::InstanceFilter>::default()
                        .with_default_system_setup(false),
                )
                .add_schedule(schedule_label, schedule)
                // and add some systems that spawn and unload lobby scenes
                .add_systems(
                    Update,
                    (
                        systems::scene::spawn_game_scene,
                        systems::scene::attach_ball_physics_components,
                        systems::scene::attach_fielder_physics_components,
                        systems::scene::attach_batter_physics_components,
                        systems::scene::attach_boundary_physics_components,
                        systems::scene::attach_wicket_physics_components,
                        // TODO lobby::systems::unload_lobby_scene,
                    ),
                );
        }

        // in all cases, add all the gameplay systems to the defined SystemSet
        app.add_systems(
            Update,
            (
                // before ticks, sync network state to physics
                // (
                //     systems::sync::sync_transforms_from_replicated::<With<ShouldTick>>,
                //     systems::sync::sync_component::<SyncVelocity, Velocity, With<ShouldTick>>,
                //     systems::sync::sync_component::<SyncImpulse, ExternalImpulse, With<ShouldTick>>,
                // ),
                // run required ticks
                self.tick_system.pipe(schedule::run_core_game_loop),
                // after running all physics ticks, sync physics state back to network components
                // (
                //     systems::sync::sync_transforms_to_replicated::<With<ShouldTick>>,
                //     systems::sync::sync_replicated::<Velocity, SyncVelocity, With<ShouldTick>>,
                //     systems::sync::sync_replicated::<ExternalImpulse, SyncImpulse, With<ShouldTick>>,
                // ),
            )
                .chain()
                .in_set(self.tick_set),
        );
    }
}
