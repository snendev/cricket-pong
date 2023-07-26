use bevy_ecs::prelude::Bundle;
use bevy_render::prelude::SpatialBundle;
use bevy_transform::prelude::Transform;

use bevy_rapier2d::prelude::{Collider, Sensor};

use cricket_pong_base::components::wicket::Wicket;

#[derive(Bundle)]
pub struct WicketPhysicsBundle {
    spatial: SpatialBundle,
    collider: Collider,
    sensor: Sensor,
}

impl WicketPhysicsBundle {
    pub fn new(transform: Transform) -> Self {
        WicketPhysicsBundle {
            spatial: SpatialBundle::from_transform(transform),
            collider: Collider::ball(Wicket::RADIUS),
            sensor: Sensor,
        }
    }
}
