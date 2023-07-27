use bevy_ecs::prelude::{Added, Commands, Entity, NextState, Query, ResMut, With};

use cricket_pong_base::{
    actions::Actions,
    components::{
        ball::{Ball, BallBundle},
        batter::{Batter, BatterBundle},
        boundary::{Boundary, BoundaryBundle},
        fielder::{Fielder, FielderBundle, FielderPosition, FielderRing, FielderTrack},
        physics::{ExternalImpulse, Transform, Velocity},
        player::{PlayerOne, PlayerTwo, Position},
        scoreboard::Scoreboard,
        wicket::{Wicket, WicketBundle},
    },
    lobby::components::{GameInstance, GameLobby},
};

use crate::{
    objects::{
        BallPhysicsBundle, BatterPhysicsBundle, BoundaryPhysicsBundle, FielderPhysicsBundle,
        WicketPhysicsBundle,
    },
    GamePhase,
};

pub(crate) fn spawn_scene(
    mut commands: Commands,
    mut state: ResMut<NextState<GamePhase>>,
    added_games_query: Query<&GameInstance, Added<GameLobby>>,
) {
    for instance in added_games_query.iter() {
        // spawn ball
        commands.spawn((BallBundle::default(), instance.clone()));

        // spawn batter
        commands.spawn((BatterBundle::default(), instance.clone()));

        // spawn fielders
        commands.spawn_batch([
            (
                FielderBundle::new(FielderPosition::Top, FielderRing::Outfield),
                instance.clone(),
            ),
            (
                FielderBundle::new(FielderPosition::Right, FielderRing::Outfield),
                instance.clone(),
            ),
            (
                FielderBundle::new(FielderPosition::Bottom, FielderRing::Outfield),
                instance.clone(),
            ),
            (
                FielderBundle::new(FielderPosition::Left, FielderRing::Outfield),
                instance.clone(),
            ),
            (
                FielderBundle::new(FielderPosition::Top, FielderRing::Infield),
                instance.clone(),
            ),
            (
                FielderBundle::new(FielderPosition::Right, FielderRing::Infield),
                instance.clone(),
            ),
            (
                FielderBundle::new(FielderPosition::Bottom, FielderRing::Infield),
                instance.clone(),
            ),
            (
                FielderBundle::new(FielderPosition::Left, FielderRing::Infield),
                instance.clone(),
            ),
        ]);

        // spawn fielder tracks
        commands.spawn_batch([
            (FielderTrack::infield(), instance.clone()),
            (FielderTrack::outfield(), instance.clone()),
        ]);

        // spawn wicket
        commands.spawn((WicketBundle::default(), instance.clone()));

        // spawn boundary
        commands.spawn((BoundaryBundle::default(), instance.clone()));

        // spawn scoreboard
        commands.spawn((Scoreboard::default(), instance.clone()));

        // spawn players
        commands.spawn((PlayerOne, Position::Batter, instance.clone()));
        commands.spawn((PlayerTwo, Position::Fielder, instance.clone()));

        todo!();
        state.set(GamePhase::Bowling);
    }
}

pub(crate) fn attach_ball_physics_components(
    mut commands: Commands,
    added_ball_query: Query<(Entity, &Transform, &Velocity, &ExternalImpulse), Added<Ball>>,
) {
    for (entity, transform, velocity, impulse) in added_ball_query.iter() {
        commands.entity(entity).insert(BallPhysicsBundle::new(
            transform.into(),
            velocity.into(),
            impulse.into(),
        ));
    }
}

pub(crate) fn attach_fielder_physics_components(
    mut commands: Commands,
    added_fielder_query: Query<(Entity, &Fielder, &Transform, &Velocity), Added<Fielder>>,
) {
    for (entity, fielder, transform, velocity) in added_fielder_query.iter() {
        commands.entity(entity).insert(FielderPhysicsBundle::new(
            fielder,
            transform.into(),
            velocity.into(),
        ));
    }
}

pub(crate) fn attach_batter_physics_components(
    mut commands: Commands,
    added_batter_query: Query<(Entity, &Transform, &Velocity), Added<Batter>>,
) {
    for (entity, transform, velocity) in added_batter_query.iter() {
        commands
            .entity(entity)
            .insert(BatterPhysicsBundle::new(transform.into(), velocity.into()));
    }
}

pub(crate) fn attach_boundary_physics_components(
    mut commands: Commands,
    added_boundary_query: Query<(Entity, &Transform), Added<Boundary>>,
) {
    for (entity, transform) in added_boundary_query.iter() {
        commands
            .entity(entity)
            .insert(BoundaryPhysicsBundle::new(transform.into()));
    }
}

pub(crate) fn attach_wicket_physics_components(
    mut commands: Commands,
    added_wicket_query: Query<(Entity, &Transform), Added<Wicket>>,
) {
    for (entity, transform) in added_wicket_query.iter() {
        commands
            .entity(entity)
            .insert(WicketPhysicsBundle::new(transform.into()));
    }
}

// should be run OnExit(MyGameState)
pub(crate) fn despawn_scene(
    mut commands: Commands,
    boundary_query: Query<Entity, With<Boundary>>,
    fielder_query: Query<Entity, With<Fielder>>,
    fielder_track_query: Query<Entity, With<FielderTrack>>,
    wicket_query: Query<Entity, With<Wicket>>,
    batter_query: Query<Entity, With<Batter>>,
    ball_query: Query<Entity, With<Ball>>,
    player_one_query: Query<Entity, With<PlayerOne>>,
    player_two_query: Query<Entity, With<PlayerTwo>>,
    scoreboard_query: Query<Entity, With<Scoreboard>>,
) {
    for entity in boundary_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in fielder_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in fielder_track_query.iter() {
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
    for entity in scoreboard_query.iter() {
        commands.entity(entity).despawn();
    }
}

// should be run OnExit(MyGameState)
pub(crate) fn deactivate_game_phase(mut state: ResMut<NextState<GamePhase>>) {
    state.set(GamePhase::Inactive);
}

pub(crate) fn cleanup_resources(mut actions: ResMut<Actions>) {
    actions.0.clear();
}
