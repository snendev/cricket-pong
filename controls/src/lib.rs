use bevy::{
    input::InputSystem as BevyInputSet,
    prelude::{
        apply_deferred, App, Component, IntoSystemConfigs, IntoSystemSetConfig, Plugin, PreUpdate,
        SystemSet,
    },
};

use leafwing_input_manager::prelude::InputManagerPlugin;

use cricket_pong_game::{base::actions::Actions, GameplayMarkerPlugin};

mod actions;
pub use actions::{BatterControl, FielderControl};

mod bundles;
pub use bundles::{
    BatterControllerBundle, BatterControllerBundle2, FielderControllerBundle,
    FielderControllerBundle2,
};

mod systems;

#[derive(Component, Debug)]
pub enum Controller {
    One,
    Two,
}

impl std::fmt::Display for Controller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, SystemSet)]
pub struct PlayerControllerSet;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        assert!(GameplayMarkerPlugin::is_added(app));
        app.add_plugins((
            InputManagerPlugin::<BatterControl>::default(),
            InputManagerPlugin::<FielderControl>::default(),
        ))
        .init_resource::<Actions>()
        .configure_set(PreUpdate, PlayerControllerSet.after(BevyInputSet))
        .add_systems(
            PreUpdate,
            (
                systems::sync_controllers,
                apply_deferred,
                systems::queue_inputs,
            )
                .chain()
                .in_set(PlayerControllerSet),
        );
    }
}
