use bevy_ecs::prelude::Bundle;
use bevy_render::prelude::SpatialBundle;
use bevy_transform::prelude::Transform;

use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, ColliderMassProperties, ExternalImpulse, RigidBody, Velocity,
};

use cricket_pong_base::components::ball::Ball;

#[derive(Bundle)]
pub struct BallPhysicsBundle {
    rigid_body: RigidBody,
    spatial: SpatialBundle,
    velocity: Velocity,
    collider: Collider,
    mass: ColliderMassProperties,
    impulse: ExternalImpulse,
    events: ActiveEvents,
}

impl BallPhysicsBundle {
    pub fn new(transform: Transform, velocity: Velocity, impulse: ExternalImpulse) -> Self {
        BallPhysicsBundle {
            rigid_body: RigidBody::Dynamic,
            spatial: SpatialBundle::from_transform(transform),
            velocity,
            collider: Collider::ball(Ball::RADIUS),
            mass: ColliderMassProperties::Mass(5.),
            impulse,
            events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}
