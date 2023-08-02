use bevy_core::Name;
use bevy_ecs::prelude::{Bundle, Component};
use bevy_transform::prelude::Transform as BevyTransform;

use naia_bevy_shared::Replicate;

use crate::components::physics::{ExternalImpulse, Transform, Velocity};

#[derive(Component, Default, Replicate)]
pub struct Ball;

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
            ball: Ball,
            transform: Transform::from(&BevyTransform::from_xyz(0., 0., 2.)),
            velocity: Velocity::default(),
            impulse: ExternalImpulse::default(),
        }
    }
}
