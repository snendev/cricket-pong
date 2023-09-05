use bevy_ecs::prelude::{Added, Commands, Entity, Query, ResMut, With, Without};
use bevy_log::{debug, info};
use bevy_transform::prelude::Transform;

use cricket_pong_base::{
    actions::Actions,
    components::{
        ball::{Ball, BallBundle},
        batter::{Batter, BatterBundle},
        boundary::{Boundary, BoundaryBundle},
        fielder::{Fielder, FielderBundle, FielderPosition, FielderRing, FielderTrackBundle},
        instance::{GameInstance, GameLobby},
        phase::GamePhase,
        player::{PlayerOne, PlayerTwo, Position},
        scoreboard::ScoreboardBundle,
        wicket::{Wicket, WicketBundle},
    },
    rapier::prelude::{ExternalImpulse, Velocity},
};

use crate::objects::{
    BallPhysicsBundle, BatterPhysicsBundle, BoundaryPhysicsBundle, FielderPhysicsBundle,
    WicketPhysicsBundle,
};

type WithAddedLobby = (Without<GamePhase>, Added<GameLobby>);

// NOTE: only spawns scenes for games that are spawned without a GamePhase
// that way external controllers (aka server) can spawn scenes without creating duplicates
pub(crate) fn spawn_game_scene(
    mut commands: Commands,
    mut added_games_query: Query<(Entity, &mut GameLobby, &GameInstance), WithAddedLobby>,
    player_one_query: Query<(Entity, &GameInstance), With<PlayerOne>>,
    player_two_query: Query<(Entity, &GameInstance), With<PlayerTwo>>,
) {
    for (lobby_entity, mut lobby, instance) in added_games_query.iter_mut() {
        debug!("Spawning game entities for instance {:?}", instance);
        // spawn ball
        commands.spawn((BallBundle::default(), instance.clone()));

        // spawn batter
        commands.spawn((BatterBundle::default(), instance.clone()));

        // spawn fielders
        for (position, ring) in [
            (FielderPosition::Top, FielderRing::Outfield),
            (FielderPosition::Right, FielderRing::Outfield),
            (FielderPosition::Bottom, FielderRing::Outfield),
            (FielderPosition::Left, FielderRing::Outfield),
            (FielderPosition::Top, FielderRing::Infield),
            (FielderPosition::Right, FielderRing::Infield),
            (FielderPosition::Bottom, FielderRing::Infield),
            (FielderPosition::Left, FielderRing::Infield),
        ] {
            commands.spawn((FielderBundle::new(position, ring), instance.clone()));
        }

        // spawn fielder tracks
        commands.spawn((FielderTrackBundle::infield(), instance.clone()));
        commands.spawn((FielderTrackBundle::outfield(), instance.clone()));

        // spawn wicket
        commands.spawn((WicketBundle::default(), instance.clone()));

        // spawn boundary
        commands.spawn((BoundaryBundle::default(), instance.clone()));

        // spawn scoreboard
        commands.spawn((ScoreboardBundle::default(), instance.clone()));

        // insert player positions
        for (entity, player_instance) in player_one_query.iter() {
            if player_instance == instance {
                info!("Player 1 found! ({:?})", entity);
                commands.entity(entity).insert(Position::Batter);
            }
        }
        for (entity, player_instance) in player_two_query.iter() {
            if player_instance == instance {
                info!("Player 2 found! ({:?})", entity);
                commands.entity(entity).insert(Position::Fielder);
            }
        }

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
        commands
            .entity(entity)
            .insert(BallPhysicsBundle::new(*transform, *velocity, *impulse));
    }
    // todo!("Check whether all these attach_physics systems have any value");
}

pub(crate) fn attach_fielder_physics_components(
    mut commands: Commands,
    added_fielder_query: Query<(Entity, &Fielder, &Transform, &Velocity), Added<Fielder>>,
) {
    for (entity, fielder, transform, velocity) in added_fielder_query.iter() {
        debug!("Fielder physics components added to entity ({:?})", entity);
        commands
            .entity(entity)
            .insert(FielderPhysicsBundle::new(fielder, *transform, *velocity));
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
            .insert(BatterPhysicsBundle::new(*transform, *velocity));
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
            .insert(BoundaryPhysicsBundle::new(*transform));
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
            .insert(WicketPhysicsBundle::new(*transform));
    }
}

// TODO: where do I put this?
pub(crate) fn _cleanup_resources(mut actions: ResMut<Actions>) {
    actions.0.clear();
}
