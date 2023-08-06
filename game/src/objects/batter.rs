use bevy_ecs::prelude::Bundle;
use bevy_math::Vec2;
use bevy_render::prelude::SpatialBundle;
use bevy_transform::prelude::Transform;

use cricket_pong_base::{
    components::batter::Batter,
    rapier::prelude::{Collider, ColliderMassProperties, MassProperties, RigidBody, Velocity},
};

#[derive(Bundle)]
pub struct BatterPhysicsBundle {
    rigid_body: RigidBody,
    spatial: SpatialBundle,
    velocity: Velocity,
    collider: Collider,
    mass: ColliderMassProperties,
}

impl BatterPhysicsBundle {
    pub fn new(transform: Transform, velocity: Velocity) -> Self {
        let radius = transform.translation.length();
        let inertia = Batter::MASS * radius * radius / 2.;
        BatterPhysicsBundle {
            rigid_body: RigidBody::KinematicVelocityBased,
            spatial: SpatialBundle::from_transform(transform),
            velocity,
            collider: Collider::cuboid(Batter::HWIDTH, Batter::HDEPTH),
            mass: ColliderMassProperties::MassProperties(MassProperties {
                local_center_of_mass: Vec2::new(-radius, 0.),
                mass: Batter::MASS,
                principal_inertia: inertia,
            }),
        }
    }
}
