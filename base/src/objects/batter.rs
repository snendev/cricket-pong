use bevy_ecs::prelude::Component;

#[derive(Component, Default)]
pub struct Batter {
    pub swing_timer: Option<f32>,
}

impl Batter {
    pub const RADIUS: f32 = 30.;
    pub const ROTATION_SPEED: f32 = std::f32::consts::FRAC_PI_6;
    pub const SWING_VELOCITY: f32 = std::f32::consts::PI * 2.;
    pub const SWING_TIME: f32 = 0.3;
}

#[derive(Component)]
pub struct Bat;

impl Bat {
    pub const HWIDTH: f32 = 30.;
    pub const HDEPTH: f32 = 5.;
}

#[derive(Component)]
pub struct Wicket;

impl Wicket {
    pub const RADIUS: f32 = Batter::RADIUS - crate::ball::Ball::RADIUS * 2.;
}
