use bevy_ecs::prelude::{Bundle, Commands};
use bevy_hierarchy::prelude::BuildChildren;
use bevy_render::prelude::SpatialBundle;
use bevy_transform::prelude::Transform;

use bevy_rapier2d::prelude::{
    ActiveEvents, AdditionalMassProperties, CoefficientCombineRule, Collider,
    ColliderMassProperties, ExternalImpulse, Restitution, RigidBody, Sensor, Velocity,
};

use cricket_pong_base::fielder::{Boundary, Fielder, FielderRing};

// the parent entity
#[derive(Bundle)]
struct FielderRingBundle {
    fielder_ring: FielderRing,
    rigid_body: RigidBody,
    spatial: SpatialBundle,
    mass: AdditionalMassProperties, // set its total mass this way
    velocity: Velocity,
    impulse: ExternalImpulse,
}

impl FielderRingBundle {
    fn new(fielder_ring: FielderRing) -> Self {
        FielderRingBundle {
            fielder_ring,
            rigid_body: RigidBody::KinematicVelocityBased,
            spatial: SpatialBundle::default(),
            mass: AdditionalMassProperties::Mass(200.),
            velocity: Velocity::zero(),
            impulse: ExternalImpulse::default(),
        }
    }

    pub fn infield() -> Self {
        Self::new(FielderRing::Infield)
    }

    pub fn outfield() -> Self {
        Self::new(FielderRing::Outfield)
    }
}
#[derive(Bundle)]
struct FielderBundle {
    fielder: Fielder,
    spatial: SpatialBundle,
    collider: Collider,
    mass: ColliderMassProperties, // each child will individually have zero mass
    restitution: Restitution,
}

impl FielderBundle {
    fn new(fielder: Fielder, transform: Transform, width: f32, height: f32) -> Self {
        FielderBundle {
            fielder,
            spatial: SpatialBundle::from_transform(transform),
            collider: Collider::cuboid(width, height),
            mass: ColliderMassProperties::Density(0.),
            restitution: Restitution {
                coefficient: 1.,
                combine_rule: CoefficientCombineRule::Max,
            },
        }
    }

    pub fn top(ring: FielderRing) -> Self {
        let fielder = Fielder::top(ring);
        let hwidth = fielder.hwidth();
        FielderBundle::new(
            fielder,
            Transform::from_xyz(0., ring.radius(), 1.),
            hwidth,
            Fielder::HDEPTH,
        )
    }

    pub fn bottom(ring: FielderRing) -> Self {
        let fielder = Fielder::bottom(ring);
        let hwidth = fielder.hwidth();
        FielderBundle::new(
            fielder,
            Transform::from_xyz(0., -ring.radius(), 1.),
            hwidth,
            Fielder::HDEPTH,
        )
    }

    pub fn left(ring: FielderRing) -> Self {
        let fielder = Fielder::left(ring);
        let hwidth = fielder.hwidth();
        FielderBundle::new(
            fielder,
            Transform::from_xyz(-ring.radius(), 0., 1.),
            Fielder::HDEPTH,
            hwidth,
        )
    }

    pub fn right(ring: FielderRing) -> Self {
        let fielder = Fielder::right(ring);
        let hwidth = fielder.hwidth();
        FielderBundle::new(
            fielder,
            Transform::from_xyz(ring.radius(), 0., 1.),
            Fielder::HDEPTH,
            hwidth,
        )
    }
}

#[derive(Bundle)]
struct BoundaryBundle {
    boundary: Boundary,
    spatial: SpatialBundle,
    collider: Collider,
    sensor: Sensor,
    events: ActiveEvents,
}

impl BoundaryBundle {
    pub fn new() -> Self {
        BoundaryBundle {
            boundary: Boundary,
            spatial: SpatialBundle::default(),
            collider: Collider::ball(Boundary::RADIUS),
            sensor: Sensor,
            events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

pub struct FieldersSpawner;

impl FieldersSpawner {
    pub fn spawn(commands: &mut Commands) {
        commands
            .spawn(FielderRingBundle::infield())
            .with_children(|parent| {
                parent.spawn(FielderBundle::top(FielderRing::Infield));
                parent.spawn(FielderBundle::bottom(FielderRing::Infield));
                parent.spawn(FielderBundle::left(FielderRing::Infield));
                parent.spawn(FielderBundle::right(FielderRing::Infield));
            });
        commands
            .spawn(FielderRingBundle::outfield())
            .with_children(|parent| {
                parent.spawn(FielderBundle::top(FielderRing::Outfield));
                parent.spawn(FielderBundle::bottom(FielderRing::Outfield));
                parent.spawn(FielderBundle::left(FielderRing::Outfield));
                parent.spawn(FielderBundle::right(FielderRing::Outfield));
            });
        commands.spawn(BoundaryBundle::new());
    }
}
