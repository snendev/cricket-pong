use bevy_ecs::{
    prelude::Entity,
    query::Changed,
    system::{Commands, Query},
};
use bevy_hierarchy::DespawnRecursiveExt;
use bevy_log::debug;

use crate::components::{GameInstance, GameLobby};

pub fn unload_lobby_scene(
    mut commands: Commands,
    game_lobby_query: Query<(Entity, &GameLobby, &GameInstance), Changed<GameLobby>>,
    instance_query: Query<(Entity, &GameInstance)>,
) {
    for (lobby_entity, lobby, lobby_instance) in game_lobby_query.iter() {
        if lobby.is_unloading() {
            debug!("Unloading game instance {}", lobby_instance);
            for (entity, instance) in instance_query.iter() {
                if instance == lobby_instance {
                    commands.entity(entity).despawn_recursive();
                }
            }
            commands.entity(lobby_entity).despawn();
        }
    }
}
