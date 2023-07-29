use bevy_ecs::prelude::{Bundle, Component};
use bevy_math::{Quat, Vec3};
use bevy_transform::prelude::Transform as BevyTransform;

use bevy_rapier2d::prelude::Velocity as RapierVelocity;

use naia_bevy_shared::{Property, Replicate, Serde};

use crate::components::physics::{Transform, Velocity};

#[derive(Clone, Copy, Debug, PartialEq, Serde)]
pub enum FielderRing {
    Infield,
    Outfield,
}

impl FielderRing {
    pub const INFIELD_RADIUS: f32 = 200.;
    pub const OUTFIELD_RADIUS: f32 = 300.;

    pub const fn radius(&self) -> f32 {
        match self {
            FielderRing::Infield => Self::INFIELD_RADIUS,
            FielderRing::Outfield => Self::OUTFIELD_RADIUS,
        }
    }
}

#[derive(Component, Replicate)]
pub struct FielderTrack {
    pub ring: Property<FielderRing>,
}

impl FielderTrack {
    pub fn infield() -> Self {
        Self::new_complete(FielderRing::Infield)
    }

    pub fn outfield() -> Self {
        Self::new_complete(FielderRing::Outfield)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serde)]
pub enum FielderPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl FielderPosition {
    // starting rotation
    fn default_rotation(&self) -> Quat {
        match self {
            FielderPosition::Top => Quat::from_rotation_z(0.),
            FielderPosition::Bottom => Quat::from_rotation_z(std::f32::consts::PI),
            FielderPosition::Left => Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
            FielderPosition::Right => Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
        }
    }

    // starting translation
    fn default_translation(&self, radius: f32) -> Vec3 {
        match self {
            FielderPosition::Top => Vec3::new(0., radius, 1.),
            FielderPosition::Bottom => Vec3::new(0., -radius, 1.),
            FielderPosition::Left => Vec3::new(-radius, 0., 1.),
            FielderPosition::Right => Vec3::new(radius, 0., 1.),
        }
    }
}

#[derive(Component, Replicate)]
pub struct Fielder {
    pub position: Property<FielderPosition>,
    pub ring: Property<FielderRing>,
}

impl Fielder {
    pub const MASS: f32 = 100.;
    pub const ROTATION_SPEED: f32 = std::f32::consts::FRAC_PI_4;
    pub const BOWL_IMPULSE: f32 = 1000.;
    pub const INFIELD_HWIDTH: f32 = 30.;
    pub const OUTFIELD_HWIDTH: f32 = 50.;
    pub const HDEPTH: f32 = 2.;

    fn new(position: FielderPosition, ring: FielderRing) -> Self {
        Fielder::new_complete(position, ring)
    }

    pub fn hwidth(&self) -> f32 {
        match *self.ring {
            FielderRing::Infield => Self::INFIELD_HWIDTH,
            FielderRing::Outfield => Self::OUTFIELD_HWIDTH,
        }
    }
}

#[derive(Bundle)]
pub struct FielderBundle {
    fielder: Fielder,
    transform: Transform,
    velocity: Velocity,
}

impl FielderBundle {
    pub fn new(position: FielderPosition, ring: FielderRing) -> Self {
        let translation = position.default_translation(ring.radius());
        let rotation = position.default_rotation();
        FielderBundle {
            fielder: Fielder::new(position, ring),
            transform: Transform::from(
                &BevyTransform::from_translation(translation).with_rotation(rotation),
            ),
            velocity: Velocity::from(&RapierVelocity::zero()),
        }
    }
}