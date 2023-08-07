use bevy_core::Name;
use bevy_ecs::prelude::{Bundle, Component};

use naia_bevy_shared::{Property, Replicate};

use crate::components::physics::{Rotation, Translation, Velocity};

#[derive(Component, Replicate)]
pub struct Batter {
    pub timer: Property<Option<f32>>,
}

impl Default for Batter {
    fn default() -> Self {
        Self::new_complete(None)
    }
}

impl std::fmt::Debug for Batter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Batter")
            .field("timer", &*self.timer)
            .finish()
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

    pub fn name() -> Name {
        Name::new("Batter")
    }
}

#[derive(Bundle)]
pub struct BatterBundle {
    name: Name,
    batter: Batter,
    translation: Translation,
    rotation: Rotation,
    velocity: Velocity,
}

impl Default for BatterBundle {
    fn default() -> Self {
        BatterBundle {
            name: Batter::name(),
            batter: Batter::default(),
            translation: Translation::new(Batter::RADIUS + Batter::HWIDTH, 0., 1.),
            rotation: Rotation::default(),
            velocity: Velocity::default(),
        }
    }
}
