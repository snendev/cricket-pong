use bevy_ecs::prelude::{Bundle, Component};

use naia_bevy_shared::Replicate;

use crate::components::{fielder::FielderRing, physics::Transform};

#[derive(Component, Default, Replicate)]
pub struct Boundary;

impl Boundary {
    pub const RADIUS: f32 = FielderRing::OUTFIELD_RADIUS + 50.;
}

#[derive(Bundle, Default)]
pub struct BoundaryBundle {
    boundary: Boundary,
    transform: Transform,
}
