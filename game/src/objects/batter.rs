use bevy_ecs::prelude::{Bundle, Commands};
use bevy_hierarchy::prelude::BuildChildren;
use bevy_render::prelude::SpatialBundle;
use bevy_transform::prelude::Transform;

use bevy_rapier2d::prelude::{Collider, ColliderMassProperties, RigidBody, Sensor, Velocity};

use cricket_pong_base::batter::{Bat, Batter, Wicket};

// the parent entity
#[derive(Bundle)]
struct BatterBundle {
    batter: Batter,
    rigid_body: RigidBody,
    spatial: SpatialBundle,
    velocity: Velocity,
}

impl BatterBundle {
    pub fn new() -> Self {
        BatterBundle {
            batter: Batter::default(),
            rigid_body: RigidBody::KinematicVelocityBased,
            spatial: SpatialBundle::from_transform(Transform::IDENTITY),
            velocity: Velocity::zero(),
        }
    }
}

#[derive(Bundle)]
struct BatBundle {
    bat: Bat,
    spatial: SpatialBundle,
    collider: Collider,
    mass: ColliderMassProperties,
}

impl BatBundle {
    pub fn new() -> Self {
        BatBundle {
            bat: Bat,
            collider: Collider::cuboid(Bat::HWIDTH, Bat::HDEPTH),
            spatial: SpatialBundle::from_transform(Transform::from_xyz(
                Batter::RADIUS + Bat::HWIDTH,
                0.,
                1.,
            )),
            mass: ColliderMassProperties::Mass(0.),
        }
    }
}

#[derive(Bundle)]
struct WicketBundle {
    wicket: Wicket,
    spatial: SpatialBundle,
    collider: Collider,
    sensor: Sensor,
}

impl WicketBundle {
    pub fn new() -> Self {
        WicketBundle {
            wicket: Wicket,
            spatial: SpatialBundle::default(),
            collider: Collider::ball(Wicket::RADIUS),
            sensor: Sensor,
        }
    }
}

pub struct BatterSpawner;

impl BatterSpawner {
    pub fn spawn(commands: &mut Commands) {
        commands.spawn(BatterBundle::new()).with_children(|parent| {
            parent.spawn((
                SpatialBundle::default(),
                Collider::ball(0.1),
                ColliderMassProperties::Mass(50.),
            ));
            parent.spawn(BatBundle::new());
        });
        commands.spawn(WicketBundle::new());
    }
}
