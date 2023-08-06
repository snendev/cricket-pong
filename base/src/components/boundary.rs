use bevy_core::Name;
use bevy_ecs::prelude::{Bundle, Component};

use naia_bevy_shared::Replicate;

use crate::components::{
    fielder::FielderRing,
    physics::{Rotation, Translation},
};

#[derive(Component, Default, Replicate)]
pub struct Boundary;

impl Boundary {
    pub const RADIUS: f32 = FielderRing::OUTFIELD_RADIUS + 50.;

    pub fn name() -> Name {
        Name::new("Boundary")
    }
}

#[derive(Bundle)]
pub struct BoundaryBundle {
    name: Name,
    boundary: Boundary,
    translation: Translation,
    rotation: Rotation,
}

impl Default for BoundaryBundle {
    fn default() -> Self {
        BoundaryBundle {
            name: Boundary::name(),
            boundary: Boundary,
            translation: Translation::default(),
            rotation: Rotation::default(),
        }
    }
}
