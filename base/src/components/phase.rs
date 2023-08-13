use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_reflect::Reflect;

#[derive(Clone, Copy, Component, Debug, Default, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum GamePhase {
    #[default]
    Bowling,
    Active,
    GameOver,
}

impl GamePhase {
    pub fn is_bowling(&self) -> bool {
        *self == GamePhase::Bowling
    }

    pub fn is_active(&self) -> bool {
        *self == GamePhase::Active
    }

    pub fn is_game_over(&self) -> bool {
        *self == GamePhase::GameOver
    }

    pub fn set_bowling(&mut self) {
        *self = GamePhase::Bowling;
    }

    pub fn set_active(&mut self) {
        *self = GamePhase::Active;
    }

    pub fn set_game_over(&mut self) {
        *self = GamePhase::GameOver;
    }
}
