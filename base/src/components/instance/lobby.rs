use bevy_core::Name;
use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_reflect::Reflect;

#[derive(Clone, Copy, Component, Debug, Default, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum GameLobby {
    #[default]
    Loading,
    Active,
    Unloading,
}

impl GameLobby {
    pub fn is_loading(&self) -> bool {
        *self == GameLobby::Loading
    }

    pub fn is_active(&self) -> bool {
        *self == GameLobby::Active
    }

    pub fn is_unloading(&self) -> bool {
        *self == GameLobby::Unloading
    }

    pub fn activate(&mut self) {
        *self = GameLobby::Active;
    }

    pub fn unload(&mut self) {
        *self = GameLobby::Unloading;
    }

    pub fn name() -> Name {
        Name::new("GameLobby")
    }
}
