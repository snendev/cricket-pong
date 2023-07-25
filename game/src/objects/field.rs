use bevy_ecs::prelude::{Bundle, Commands};
use bevy_math::Vec2;
use bevy_render::prelude::SpatialBundle;
use bevy_transform::prelude::Transform;

use bevy_rapier2d::prelude::{
    CoefficientCombineRule, Collider, ColliderMassProperties, MassProperties, Restitution,
    RigidBody, Sensor, Velocity,
};

use cricket_pong_base::fielder::{Boundary, Fielder, FielderRing};

#[derive(Bundle)]
struct FielderBundle {
    fielder: Fielder,
    rigid_body: RigidBody,
    spatial: SpatialBundle,
    velocity: Velocity,
    collider: Collider,
    mass: ColliderMassProperties, // each child will individually have zero mass
    restitution: Restitution,
}

impl FielderBundle {
    fn new(fielder: Fielder) -> Self {
        let radius = fielder.ring.radius();
        let inertia = Fielder::MASS * radius * radius / 2.;
        let translation = fielder.translation();
        let rotation = fielder.rotation();
        let hwidth = fielder.hwidth();
        FielderBundle {
            fielder,
            rigid_body: RigidBody::KinematicVelocityBased,
            velocity: Velocity::zero(),
            spatial: SpatialBundle::from_transform(
                Transform::from_translation(translation).with_rotation(rotation),
            ),
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

    pub fn top(ring: FielderRing) -> Self {
        FielderBundle::new(Fielder::top(ring))
    }

    pub fn bottom(ring: FielderRing) -> Self {
        FielderBundle::new(Fielder::bottom(ring))
    }

    pub fn left(ring: FielderRing) -> Self {
        FielderBundle::new(Fielder::left(ring))
    }

    pub fn right(ring: FielderRing) -> Self {
        FielderBundle::new(Fielder::right(ring))
    }
}

#[derive(Bundle)]
struct BoundaryBundle {
    boundary: Boundary,
    spatial: SpatialBundle,
    collider: Collider,
    sensor: Sensor,
}

impl BoundaryBundle {
    pub fn new() -> Self {
        BoundaryBundle {
            boundary: Boundary,
            spatial: SpatialBundle::default(),
            collider: Collider::ball(Boundary::RADIUS),
            sensor: Sensor,
        }
    }
}

pub struct FieldersSpawner;

impl FieldersSpawner {
    pub fn spawn(commands: &mut Commands) {
        commands.spawn(FielderRing::Infield);
        commands.spawn(FielderBundle::top(FielderRing::Infield));
        commands.spawn(FielderBundle::bottom(FielderRing::Infield));
        commands.spawn(FielderBundle::left(FielderRing::Infield));
        commands.spawn(FielderBundle::right(FielderRing::Infield));

        commands.spawn(FielderRing::Outfield);
        commands.spawn(FielderBundle::top(FielderRing::Outfield));
        commands.spawn(FielderBundle::bottom(FielderRing::Outfield));
        commands.spawn(FielderBundle::left(FielderRing::Outfield));
        commands.spawn(FielderBundle::right(FielderRing::Outfield));

        commands.spawn(BoundaryBundle::new());
    }
}
