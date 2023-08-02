use bevy_core::Name;
use bevy_ecs::prelude::{Bundle, Component};

use naia_bevy_shared::Replicate;

use crate::components::{ball::Ball, batter::Batter, physics::Transform};

#[derive(Component, Default, Replicate)]
pub struct Wicket;

impl Wicket {
    pub const RADIUS: f32 = Batter::RADIUS - Ball::RADIUS * 2.;

    pub fn name() -> Name {
        Name::new("Wicket")
    }
}

#[derive(Bundle)]
pub struct WicketBundle {
    name: Name,
    wicket: Wicket,
    transform: Transform,
}

impl Default for WicketBundle {
    fn default() -> Self {
        WicketBundle {
            name: Wicket::name(),
            wicket: Wicket,
            transform: Transform::default(),
        }
    }
}
