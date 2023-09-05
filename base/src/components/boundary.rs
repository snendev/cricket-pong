use bevy_core::Name;
use bevy_ecs::prelude::{Bundle, Component, ReflectComponent};
use bevy_reflect::Reflect;
use bevy_transform::prelude::Transform;

use crate::components::fielder::FielderRing;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
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
    transform: Transform,
}

impl Default for BoundaryBundle {
    fn default() -> Self {
        BoundaryBundle {
            name: Boundary::name(),
            boundary: Boundary,
            transform: Transform::default(),
        }
    }
}
