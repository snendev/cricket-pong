use bevy_ecs::prelude::{Bundle, Entity};

use crate::components::{GameInstance, GameLobby, Player};

#[derive(Bundle)]
pub struct GameBundle {
    pub lobby: GameLobby,
    pub instance: GameInstance,
}

impl GameBundle {
    pub fn new(id: u64) -> Self {
        GameBundle {
            lobby: GameLobby,
            instance: GameInstance::new(id),
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub instance: GameInstance,
}

impl PlayerBundle {
    pub fn new(player: u64, instance: u64) -> Self {
        PlayerBundle {
            player: Player::new(player),
            instance: GameInstance::new(instance),
        }
    }
}

// useful query types

pub type GameEntityTuple<'a> = (Entity, &'a GameInstance);
pub type GamePlayerTuple<'a> = (Entity, &'a GameInstance, &'a Player);
