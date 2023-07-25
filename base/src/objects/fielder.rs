use bevy_ecs::prelude::Component;
use bevy_math::{Quat, Vec3};

#[derive(Clone, Copy, Component, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FielderPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl FielderPosition {
    // starting rotation
    fn rotation(&self) -> Quat {
        match self {
            FielderPosition::Top => Quat::from_rotation_z(0.),
            FielderPosition::Bottom => Quat::from_rotation_z(std::f32::consts::PI),
            FielderPosition::Left => Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
            FielderPosition::Right => Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
        }
    }

    // starting translation
    fn translation(&self, radius: f32) -> Vec3 {
        match self {
            FielderPosition::Top => Vec3::new(0., radius, 1.),
            FielderPosition::Bottom => Vec3::new(0., -radius, 1.),
            FielderPosition::Left => Vec3::new(-radius, 0., 1.),
            FielderPosition::Right => Vec3::new(radius, 0., 1.),
        }
    }
}

#[derive(Component)]
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

    const fn new(position: FielderPosition, ring: FielderRing) -> Self {
        Fielder { position, ring }
    }

    pub const fn hwidth(&self) -> f32 {
        match self.ring {
            FielderRing::Infield => Self::INFIELD_HWIDTH,
            FielderRing::Outfield => Self::OUTFIELD_HWIDTH,
        }
    }

    pub const fn top(ring: FielderRing) -> Self {
        Fielder::new(FielderPosition::Top, ring)
    }

    pub const fn bottom(ring: FielderRing) -> Self {
        Fielder::new(FielderPosition::Bottom, ring)
    }

    pub const fn left(ring: FielderRing) -> Self {
        Fielder::new(FielderPosition::Left, ring)
    }

    pub const fn right(ring: FielderRing) -> Self {
        Fielder::new(FielderPosition::Right, ring)
    }

    pub fn rotation(&self) -> Quat {
        self.position.rotation()
    }

    pub fn translation(&self) -> Vec3 {
        self.position.translation(self.ring.radius())
    }
}

#[derive(Component)]
pub struct Boundary;

impl Boundary {
    pub const RADIUS: f32 = FielderRing::OUTFIELD_RADIUS + 50.;
}
