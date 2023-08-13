use bevy_core::Name;
use bevy_ecs::prelude::{Bundle, Component, ReflectComponent};
use bevy_reflect::Reflect;
use bevy_transform::prelude::Transform;

use crate::rapier::prelude::{ExternalImpulse, Velocity};

#[derive(Clone, Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Ball {
    pub passes: usize,
}

impl Ball {
    pub const RADIUS: f32 = 8.;

    pub fn name() -> Name {
        Name::new("Ball")
    }
}

#[derive(Bundle)]
pub struct BallBundle {
    name: Name,
    ball: Ball,
    transform: Transform,
    impulse: ExternalImpulse,
    velocity: Velocity,
}

impl Default for BallBundle {
    fn default() -> Self {
        BallBundle {
            name: Ball::name(),
            ball: Ball::default(),
            transform: Transform::from_xyz(0., 0., 2.),
            velocity: Velocity::default(),
            impulse: ExternalImpulse::default(),
        }
    }
}
