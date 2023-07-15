use bevy_ecs::prelude::Bundle;
use bevy_transform::prelude::Transform;

use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, ColliderMassProperties, ExternalImpulse, RigidBody, Velocity,
};

use cricket_pong_base::ball::Ball;

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    rigid_body: RigidBody,
    transform: Transform,
    velocity: Velocity,
    collider: Collider,
    mass: ColliderMassProperties,
    impulse: ExternalImpulse,
    events: ActiveEvents,
}

impl BallBundle {
    pub fn new(transform: Transform) -> Self {
        BallBundle {
            ball: Ball,
            rigid_body: RigidBody::Dynamic,
            transform,
            velocity: Velocity::default(),
            collider: Collider::ball(Ball::RADIUS),
            mass: ColliderMassProperties::Mass(5.),
            impulse: ExternalImpulse::default(),
            events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}
