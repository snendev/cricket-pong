use bevy_ecs::prelude::Component;

#[derive(Clone, Copy, Component, Debug, PartialEq)]
pub enum FielderRing {
    Infield,
    Outfield,
}

impl FielderRing {
    pub const INFIELD_RADIUS: f32 = 200.;
    pub const OUTFIELD_RADIUS: f32 = 300.;

    pub fn radius(&self) -> f32 {
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

#[derive(Component)]
pub struct Fielder {
    pub position: FielderPosition,
    pub ring: FielderRing,
}

impl Fielder {
    pub const ROTATION_SPEED: f32 = std::f32::consts::FRAC_PI_4;
    pub const BOWL_IMPULSE: f32 = 1000.;
    pub const INFIELD_HWIDTH: f32 = 30.;
    pub const OUTFIELD_HWIDTH: f32 = 50.;
    pub const HDEPTH: f32 = 2.;

    fn new(position: FielderPosition, ring: FielderRing) -> Self {
        Fielder { position, ring }
    }

    pub fn hwidth(&self) -> f32 {
        match self.ring {
            FielderRing::Infield => Self::INFIELD_HWIDTH,
            FielderRing::Outfield => Self::OUTFIELD_HWIDTH,
        }
    }

    pub fn top(ring: FielderRing) -> Self {
        Fielder::new(FielderPosition::Top, ring)
    }

    pub fn bottom(ring: FielderRing) -> Self {
        Fielder::new(FielderPosition::Bottom, ring)
    }

    pub fn left(ring: FielderRing) -> Self {
        Fielder::new(FielderPosition::Left, ring)
    }

    pub fn right(ring: FielderRing) -> Self {
        Fielder::new(FielderPosition::Right, ring)
    }
}

#[derive(Component)]
pub struct Boundary;

impl Boundary {
    pub const RADIUS: f32 = FielderRing::OUTFIELD_RADIUS + 50.;
}
