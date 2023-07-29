use bevy_ecs::prelude::{Added, Commands, Entity, Query, ResMut};

use bevy_log::debug;
use cricket_pong_base::{
    actions::Actions,
    components::{
        ball::{Ball, BallBundle},
        batter::{Batter, BatterBundle},
        boundary::{Boundary, BoundaryBundle},
        fielder::{Fielder, FielderBundle, FielderPosition, FielderRing, FielderTrack},
        phase::GamePhase,
        physics::{ExternalImpulse, Transform, Velocity},
        player::{PlayerOne, PlayerTwo, Position},
        scoreboard::Scoreboard,
        wicket::{Wicket, WicketBundle},
    },
    lobby::components::{GameInstance, GameLobby},
};

use crate::objects::{
    BallPhysicsBundle, BatterPhysicsBundle, BoundaryPhysicsBundle, FielderPhysicsBundle,
    WicketPhysicsBundle,
};

pub(crate) fn spawn_scene(
    mut commands: Commands,
    mut added_games_query: Query<(Entity, &mut GameLobby, &GameInstance), Added<GameLobby>>,
) {
    for (lobby_entity, mut lobby, instance) in added_games_query.iter_mut() {
        debug!("Spawning game entities for instance {}", instance);
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

        lobby.activate();
        commands.entity(lobby_entity).insert(GamePhase::default());
    }
}

pub(crate) fn attach_ball_physics_components(
    mut commands: Commands,
    added_ball_query: Query<(Entity, &Transform, &Velocity, &ExternalImpulse), Added<Ball>>,
) {
    for (entity, transform, velocity, impulse) in added_ball_query.iter() {
        debug!("Ball physics components added to entity ({:?})", entity);
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
        debug!("Fielder physics components added to entity ({:?})", entity);
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
        debug!("Batter physics components added to entity ({:?})", entity);
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
        debug!("Boundary physics components added to entity ({:?})", entity);
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
        debug!("Wicket physics components added to entity ({:?})", entity);
        commands
            .entity(entity)
            .insert(WicketPhysicsBundle::new(transform.into()));
    }
}

// TODO: where do I put this?
pub(crate) fn _cleanup_resources(mut actions: ResMut<Actions>) {
    actions.0.clear();
}
