use bevy_ecs::prelude::{Bundle, Component};
use naia_bevy_shared::Replicate;

use crate::components::physics::{ExternalImpulse, Transform, Velocity};

#[derive(Component, Default, Replicate)]
pub struct Ball;

impl Ball {
    pub const RADIUS: f32 = 8.;
}

#[derive(Bundle, Default)]
pub struct BallBundle {
    ball: Ball,
    transform: Transform,
    impulse: ExternalImpulse,
    velocity: Velocity,
}
