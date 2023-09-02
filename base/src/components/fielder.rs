use bevy_core::Name;
use bevy_ecs::prelude::{Bundle, Component, ReflectComponent};
use bevy_math::{Quat, Vec3};
use bevy_reflect::Reflect;
use bevy_transform::prelude::Transform;

use crate::rapier::prelude::Velocity;

#[derive(Clone, Copy, Component, Debug, Default, PartialEq, Reflect)]
pub enum FielderRing {
    #[default]
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

#[derive(Clone, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct FielderTrack {
    pub ring: FielderRing,
}

impl FielderTrack {
    fn new(ring: FielderRing) -> Self {
        FielderTrack { ring }
    }

    pub fn infield() -> Self {
        FielderTrack::new(FielderRing::Infield)
    }

    pub fn outfield() -> Self {
        FielderTrack::new(FielderRing::Outfield)
    }

    pub fn name() -> Name {
        Name::new("FielderTrack")
    }
}

#[derive(Bundle)]
pub struct FielderTrackBundle {
    name: Name,
    track: FielderTrack,
}

impl FielderTrackBundle {
    pub fn infield() -> Self {
        FielderTrackBundle {
            name: FielderTrack::name(),
            track: FielderTrack::infield(),
        }
    }

    pub fn outfield() -> Self {
        FielderTrackBundle {
            name: FielderTrack::name(),
            track: FielderTrack::outfield(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub enum FielderPosition {
    #[default]
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

#[derive(Clone, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Fielder {
    pub position: FielderPosition,
    pub ring: FielderRing,
}

impl Fielder {
    pub const MASS: f32 = 100.;
    pub const ROTATION_SPEED: f32 = std::f32::consts::FRAC_PI_4;
    pub const BOWL_IMPULSE: f32 = 1000.;
    pub const INFIELD_HWIDTH: f32 = 30.;
    pub const OUTFIELD_HWIDTH: f32 = 50.;
    pub const HDEPTH: f32 = 2.;

    pub fn new(position: FielderPosition, ring: FielderRing) -> Self {
        Fielder { position, ring }
    }

    pub fn hwidth(&self) -> f32 {
        match self.ring {
            FielderRing::Infield => Self::INFIELD_HWIDTH,
            FielderRing::Outfield => Self::OUTFIELD_HWIDTH,
        }
    }

    pub fn name() -> Name {
        Name::new("Fielder")
    }
}

#[derive(Bundle)]
pub struct FielderBundle {
    name: Name,
    fielder: Fielder,
    transform: Transform,
    velocity: Velocity,
}

impl FielderBundle {
    pub fn new(position: FielderPosition, ring: FielderRing) -> Self {
        let translation = position.default_translation(ring.radius());
        let rotation = position.default_rotation();
        FielderBundle {
            name: Fielder::name(),
            fielder: Fielder::new(position, ring),
            transform: Transform::from_translation(translation).with_rotation(rotation),
            velocity: Velocity::zero(),
        }
    }
}
