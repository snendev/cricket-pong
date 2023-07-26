use bevy_ecs::prelude::{Bundle, Component};
use bevy_transform::prelude::Transform as BevyTransform;

use naia_bevy_shared::{Property, Replicate};

use crate::components::physics::{Transform, Velocity};

#[derive(Component, Replicate)]
pub struct Batter {
    pub timer: Property<Option<f32>>,
}

impl Default for Batter {
    fn default() -> Self {
        Self::new_complete(None)
    }
}

impl Batter {
    pub const RADIUS: f32 = 30.;
    pub const ROTATION_SPEED: f32 = std::f32::consts::FRAC_PI_6;
    pub const SWING_VELOCITY: f32 = std::f32::consts::PI * 2.;
    pub const SWING_TIME: f32 = 0.3;
    pub const HWIDTH: f32 = 25.;
    pub const HDEPTH: f32 = 5.;
    pub const MASS: f32 = 50.;
}

#[derive(Bundle)]
pub struct BatterBundle {
    batter: Batter,
    transform: Transform,
    velocity: Velocity,
}

impl Default for BatterBundle {
    fn default() -> Self {
        BatterBundle {
            batter: Batter::default(),
            transform: Transform::from(&BevyTransform::from_xyz(
                Batter::RADIUS + Batter::HWIDTH,
                0.,
                1.,
            )),
            velocity: Velocity::default(),
        }
    }
}
