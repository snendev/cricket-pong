use bevy_ecs::prelude::Component;

use naia_bevy_shared::{Property, Replicate, Serde};

#[derive(Clone, Copy, Debug, Default, PartialEq, Serde)]
pub enum GamePhaseKind {
    #[default]
    Bowling,
    Active,
    GameOver,
}

#[derive(Component, Replicate)]
pub struct GamePhase {
    kind: Property<GamePhaseKind>,
}

impl From<GamePhaseKind> for GamePhase {
    fn from(kind: GamePhaseKind) -> Self {
        GamePhase::new_complete(kind)
    }
}

impl Default for GamePhase {
    fn default() -> Self {
        GamePhase::new_complete(GamePhaseKind::default())
    }
}

impl GamePhase {
    pub fn inner(&self) -> GamePhaseKind {
        *self.kind
    }

    pub fn is_bowling(&self) -> bool {
        *self.kind == GamePhaseKind::Bowling
    }

    pub fn is_active(&self) -> bool {
        *self.kind == GamePhaseKind::Active
    }

    pub fn is_game_over(&self) -> bool {
        *self.kind == GamePhaseKind::GameOver
    }

    pub fn set_bowling(&mut self) {
        *self.kind = GamePhaseKind::Bowling;
    }

    pub fn set_active(&mut self) {
        *self.kind = GamePhaseKind::Active;
    }

    pub fn set_game_over(&mut self) {
        *self.kind = GamePhaseKind::GameOver;
    }
}

impl std::fmt::Debug for GamePhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GamePhase")
            .field("kind", &*self.kind)
            .finish()
    }
}
