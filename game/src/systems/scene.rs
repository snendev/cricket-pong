use bevy_ecs::prelude::{Commands, Entity, NextState, Query, ResMut, With};
use bevy_transform::prelude::Transform;

use cricket_pong_base::{
    ball::Ball,
    batter::{Batter, Wicket},
    fielder::{Boundary, Fielder, FielderRing},
    Over, PlayerOne, PlayerTwo,
};

use crate::{
    actions::Actions,
    objects::{ball::BallBundle, batter::BatterSpawner, field::FieldersSpawner},
    GamePhase,
};

// should be run OnEnter(MyGameState)
pub(crate) fn spawn_scene(mut commands: Commands, mut state: ResMut<NextState<GamePhase>>) {
    commands.spawn(BallBundle::new(Transform::from_xyz(0., 0., 1.)));
    FieldersSpawner::spawn(&mut commands);
    BatterSpawner::spawn(&mut commands);
    state.set(GamePhase::Preparing);
}

// should be run OnExit(MyGameState)
pub(crate) fn despawn_scene(
    mut commands: Commands,
    boundary_query: Query<Entity, With<Boundary>>,
    fielder_query: Query<Entity, With<Fielder>>,
    fielder_ring_query: Query<Entity, With<FielderRing>>,
    wicket_query: Query<Entity, With<Wicket>>,
    batter_query: Query<Entity, With<Batter>>,
    ball_query: Query<Entity, With<Ball>>,
    player_one_query: Query<Entity, With<PlayerOne>>,
    player_two_query: Query<Entity, With<PlayerTwo>>,
) {
    for entity in boundary_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in fielder_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in fielder_ring_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in wicket_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in batter_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in ball_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in player_one_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in player_two_query.iter() {
        commands.entity(entity).despawn();
    }
}

// should be run OnExit(MyGameState)
pub(crate) fn deactivate_game_phase(mut state: ResMut<NextState<GamePhase>>) {
    state.set(GamePhase::Inactive);
}

pub(crate) fn cleanup_resources(mut overs: ResMut<Over>, mut actions: ResMut<Actions>) {
    overs.clear();
    actions.0.clear();
}
