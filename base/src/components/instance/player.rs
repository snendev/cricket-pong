use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_reflect::Reflect;

// player ID

#[derive(Clone, Component, Debug, Default, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct PlayerID {
    pub id: u64,
}

impl PlayerID {
    pub fn new(id: u64) -> Self {
        PlayerID { id }
    }
}
