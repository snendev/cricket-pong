use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_reflect::Reflect;

#[derive(Clone, Component, Debug, Default, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct GameInstance {
    pub id: u64,
}

impl GameInstance {
    pub fn new(id: u64) -> Self {
        GameInstance { id }
    }
}
