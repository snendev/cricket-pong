use bevy_ecs::prelude::{Bundle, Component};

use naia_bevy_shared::Replicate;

use crate::components::{ball::Ball, batter::Batter, physics::Transform};

#[derive(Component, Default, Replicate)]
pub struct Wicket;

impl Wicket {
    pub const RADIUS: f32 = Batter::RADIUS - Ball::RADIUS * 2.;
}

#[derive(Bundle, Default)]
pub struct WicketBundle {
    wicket: Wicket,
    transform: Transform,
}
