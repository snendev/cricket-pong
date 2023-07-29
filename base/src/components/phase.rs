use bevy_ecs::prelude::Component;

#[derive(Clone, Copy, Component, Debug, Default, PartialEq)]
pub enum GamePhase {
    #[default]
    Bowling,
    Active,
    GameOver,
}
