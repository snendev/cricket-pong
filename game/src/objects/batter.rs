use bevy_ecs::prelude::{Bundle, Commands};
use bevy_math::Vec2;
use bevy_render::prelude::SpatialBundle;
use bevy_transform::prelude::Transform;

use bevy_rapier2d::prelude::{
    Collider, ColliderMassProperties, MassProperties, RigidBody, Sensor, Velocity,
};

use cricket_pong_base::batter::{Batter, Wicket};

#[derive(Bundle)]
struct BatterBundle {
    batter: Batter,
    rigid_body: RigidBody,
    spatial: SpatialBundle,
    velocity: Velocity,
    collider: Collider,
    mass: ColliderMassProperties,
}

impl BatterBundle {
    pub fn new() -> Self {
        let bat_true_radius = Batter::RADIUS + Batter::HWIDTH;
        let inertia = Batter::MASS * bat_true_radius * bat_true_radius / 2.;
        BatterBundle {
            batter: Batter::default(),
            rigid_body: RigidBody::KinematicVelocityBased,
            spatial: SpatialBundle::from_transform(Transform::from_xyz(bat_true_radius, 0., 1.)),
            velocity: Velocity::zero(),
            collider: Collider::cuboid(Batter::HWIDTH, Batter::HDEPTH),
            mass: ColliderMassProperties::MassProperties(MassProperties {
                local_center_of_mass: Vec2::new(-bat_true_radius, 0.),
                mass: Batter::MASS,
                principal_inertia: inertia,
            }),
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
        commands.spawn(BatterBundle::new());
        commands.spawn(WicketBundle::new());
    }
}
