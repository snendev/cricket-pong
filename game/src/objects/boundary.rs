use bevy_ecs::prelude::Bundle;
use bevy_render::prelude::SpatialBundle;
use bevy_transform::prelude::Transform;

use cricket_pong_base::{
    components::boundary::Boundary,
    rapier::prelude::{Collider, Sensor},
};

#[derive(Bundle)]
pub struct BoundaryPhysicsBundle {
    spatial: SpatialBundle,
    collider: Collider,
    sensor: Sensor,
}

impl BoundaryPhysicsBundle {
    pub fn new(transform: Transform) -> Self {
        BoundaryPhysicsBundle {
            spatial: SpatialBundle::from_transform(transform),
            collider: Collider::ball(Boundary::RADIUS),
            sensor: Sensor,
        }
    }
}
