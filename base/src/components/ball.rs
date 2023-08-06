use bevy_core::Name;
use bevy_ecs::prelude::{Bundle, Component};

use naia_bevy_shared::Replicate;

use crate::components::physics::{ExternalImpulse, Rotation, Translation, Velocity};

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
    translation: Translation,
    rotation: Rotation,
    impulse: ExternalImpulse,
    velocity: Velocity,
}

impl Default for BallBundle {
    fn default() -> Self {
        BallBundle {
            name: Ball::name(),
            ball: Ball,
            translation: Translation::new(0., 0., 2.),
            rotation: Rotation::default(),
            velocity: Velocity::default(),
            impulse: ExternalImpulse::default(),
        }
    }
}
