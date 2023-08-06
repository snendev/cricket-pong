use bevy_ecs::prelude::Bundle;
use bevy_math::Vec2;
use bevy_render::prelude::SpatialBundle;
use bevy_transform::prelude::Transform;

use cricket_pong_base::{
    components::fielder::Fielder,
    rapier::prelude::{
        CoefficientCombineRule, Collider, ColliderMassProperties, MassProperties, Restitution,
        RigidBody, Velocity,
    },
};

#[derive(Bundle)]
pub struct FielderPhysicsBundle {
    rigid_body: RigidBody,
    spatial: SpatialBundle,
    velocity: Velocity,
    collider: Collider,
    mass: ColliderMassProperties, // each child will individually have zero mass
    restitution: Restitution,
}

impl FielderPhysicsBundle {
    pub fn new(fielder: &Fielder, transform: Transform, velocity: Velocity) -> Self {
        let radius = fielder.ring.radius();
        let inertia = Fielder::MASS * radius * radius / 2.;
        let hwidth = fielder.hwidth();
        FielderPhysicsBundle {
            rigid_body: RigidBody::KinematicVelocityBased,
            velocity,
            spatial: SpatialBundle::from_transform(transform),
            collider: Collider::cuboid(hwidth, Fielder::HDEPTH),
            mass: ColliderMassProperties::MassProperties(MassProperties {
                local_center_of_mass: Vec2::new(0., -radius),
                mass: Fielder::MASS,
                principal_inertia: inertia,
            }),
            restitution: Restitution {
                coefficient: 1.,
                combine_rule: CoefficientCombineRule::Max,
            },
        }
    }
}
