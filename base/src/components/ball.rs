use bevy_core::Name;
use bevy_ecs::prelude::{Bundle, Component};

use naia_bevy_shared::{Property, Replicate};

use crate::components::physics::{ExternalImpulse, Rotation, Translation, Velocity};

#[derive(Component, Replicate)]
pub struct Ball {
    pub passes: Property<usize>,
}

impl Ball {
    pub const RADIUS: f32 = 8.;

    pub fn name() -> Name {
        Name::new("Ball")
    }
}

impl Default for Ball {
    fn default() -> Self {
        Ball::new_complete(0)
    }
}

impl std::fmt::Debug for Ball {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ball")
            .field("passes", &*self.passes)
            .finish()
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
            ball: Ball::default(),
            translation: Translation::new(0., 0., 2.),
            rotation: Rotation::default(),
            velocity: Velocity::default(),
            impulse: ExternalImpulse::default(),
        }
    }
}
