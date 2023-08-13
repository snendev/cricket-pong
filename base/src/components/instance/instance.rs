use bevy_ecs::prelude::Component;

use naia_bevy_shared::{Property, Replicate};

#[derive(Component, Replicate)]
pub struct GameInstance {
    pub id: Property<u64>,
}

impl GameInstance {
    pub fn new(id: u64) -> Self {
        GameInstance::new_complete(id)
    }
}

impl PartialEq for GameInstance {
    fn eq(&self, other: &Self) -> bool {
        *self.id == *other.id
    }
}

impl std::fmt::Debug for GameInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameInstance")
            .field("id", &*self.id)
            .finish()
    }
}

impl std::fmt::Display for GameInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self.id)
    }
}
