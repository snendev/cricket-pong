use bevy_core::Name;
use bevy_ecs::prelude::{Bundle, Component, ReflectComponent};
use bevy_reflect::Reflect;
use bevy_transform::prelude::Transform;

use crate::rapier::prelude::Velocity;

#[derive(Clone, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Batter {
    pub timer: Option<f32>,
}

impl Batter {
    pub const RADIUS: f32 = 30.;
    pub const ROTATION_SPEED: f32 = std::f32::consts::FRAC_PI_6;
    pub const SWING_VELOCITY: f32 = std::f32::consts::PI * 2.;
    pub const SWING_TIME: f32 = 0.3;
    pub const HWIDTH: f32 = 25.;
    pub const HDEPTH: f32 = 5.;
    pub const MASS: f32 = 50.;

    pub fn name() -> Name {
        Name::new("Batter")
    }
}

#[derive(Bundle)]
pub struct BatterBundle {
    name: Name,
    batter: Batter,
    transform: Transform,
    velocity: Velocity,
}

impl Default for BatterBundle {
    fn default() -> Self {
        BatterBundle {
            name: Batter::name(),
            batter: Batter::default(),
            transform: Transform::from_xyz(Batter::RADIUS + Batter::HWIDTH, 0., 1.),
            velocity: Velocity::default(),
        }
    }
}
