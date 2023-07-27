use bevy_ecs::prelude::Component;

use naia_bevy_shared::{Property, Replicate};

// player ID

#[derive(Component, Replicate)]
pub struct Player {
    pub id: Property<u64>,
}

impl Player {
    pub fn new(id: u64) -> Self {
        Player::new_complete(id)
    }
}
