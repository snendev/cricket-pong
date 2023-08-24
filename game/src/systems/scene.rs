use bevy_ecs::prelude::{Added, Commands, Entity, Or, Query, ResMut, With, Without};
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

use crate::{
    objects::{
        BallPhysicsBundle, BatterPhysicsBundle, BoundaryPhysicsBundle, FielderPhysicsBundle,
        WicketPhysicsBundle,
    },
    ShouldTick,
};

type WithAddedLobby = (
    With<ShouldTick>,
    Without<GamePhase>,
    Or<(Added<GameLobby>, Added<ShouldTick>)>,
);

type WithPlayer<Player> = (With<Player>, With<ShouldTick>);

// NOTE: only spawns scenes for games that are hosted by this app
// (via ShouldTick on the GameLobby and Player components)
pub(crate) fn spawn_scene(
    mut commands: Commands,
    mut added_games_query: Query<(Entity, &mut GameLobby, &GameInstance), WithAddedLobby>,
    player_one_query: Query<(Entity, &GameInstance), WithPlayer<PlayerOne>>,
    player_two_query: Query<(Entity, &GameInstance), WithPlayer<PlayerTwo>>,
) -> Vec<(GameInstance, Vec<Entity>)> {
    let mut entities = Vec::new();
    for (lobby_entity, mut lobby, instance) in added_games_query.iter_mut() {
        let mut lobby_entities = Vec::new();

        debug!("Spawning game entities for instance {:?}", instance);
        // spawn ball
        lobby_entities.push(
            commands
                .spawn((BallBundle::default(), instance.clone(), ShouldTick))
                .id(),
        );

        // spawn batter
        lobby_entities.push(
            commands
                .spawn((BatterBundle::default(), instance.clone(), ShouldTick))
                .id(),
        );

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
            lobby_entities.push(
                commands
                    .spawn((
                        FielderBundle::new(position, ring),
                        instance.clone(),
                        ShouldTick,
                    ))
                    .id(),
            );
        }

        // spawn fielder tracks
        lobby_entities.push(
            commands
                .spawn((FielderTrackBundle::infield(), instance.clone(), ShouldTick))
                .id(),
        );
        lobby_entities.push(
            commands
                .spawn((FielderTrackBundle::outfield(), instance.clone(), ShouldTick))
                .id(),
        );

        // spawn wicket
        lobby_entities.push(
            commands
                .spawn((WicketBundle::default(), instance.clone(), ShouldTick))
                .id(),
        );

        // spawn boundary
        lobby_entities.push(
            commands
                .spawn((BoundaryBundle::default(), instance.clone(), ShouldTick))
                .id(),
        );

        // spawn scoreboard
        lobby_entities.push(
            commands
                .spawn((ScoreboardBundle::default(), instance.clone(), ShouldTick))
                .id(),
        );

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

        entities.push((instance.clone(), lobby_entities));

        lobby.activate();
        commands.entity(lobby_entity).insert(GamePhase::default());
    }

    entities
}

type WithAddedBall = (Added<Ball>, With<ShouldTick>);

pub(crate) fn attach_ball_physics_components(
    mut commands: Commands,
    added_ball_query: Query<(Entity, &Transform, &Velocity, &ExternalImpulse), WithAddedBall>,
) {
    for (entity, transform, velocity, impulse) in added_ball_query.iter() {
        debug!("Ball physics components added to entity ({:?})", entity);
        commands
            .entity(entity)
            .insert(BallPhysicsBundle::new(*transform, *velocity, *impulse));
    }
    // todo!("Check whether all these attach_physics systems have any value");
}

type WithAddedFielder = (Added<Fielder>, With<ShouldTick>);

pub(crate) fn attach_fielder_physics_components(
    mut commands: Commands,
    added_fielder_query: Query<(Entity, &Fielder, &Transform, &Velocity), WithAddedFielder>,
) {
    for (entity, fielder, transform, velocity) in added_fielder_query.iter() {
        debug!("Fielder physics components added to entity ({:?})", entity);
        commands
            .entity(entity)
            .insert(FielderPhysicsBundle::new(fielder, *transform, *velocity));
    }
}

type WithAddedBatter = (Added<Batter>, With<ShouldTick>);

pub(crate) fn attach_batter_physics_components(
    mut commands: Commands,
    added_batter_query: Query<(Entity, &Transform, &Velocity), WithAddedBatter>,
) {
    for (entity, transform, velocity) in added_batter_query.iter() {
        debug!("Batter physics components added to entity ({:?})", entity);
        commands
            .entity(entity)
            .insert(BatterPhysicsBundle::new(*transform, *velocity));
    }
}

type WithAddedBoundary = (Added<Boundary>, With<ShouldTick>);

pub(crate) fn attach_boundary_physics_components(
    mut commands: Commands,
    added_boundary_query: Query<(Entity, &Transform), WithAddedBoundary>,
) {
    for (entity, transform) in added_boundary_query.iter() {
        debug!("Boundary physics components added to entity ({:?})", entity);
        commands
            .entity(entity)
            .insert(BoundaryPhysicsBundle::new(*transform));
    }
}

type WithAddedWicket = (Added<Wicket>, With<ShouldTick>);

pub(crate) fn attach_wicket_physics_components(
    mut commands: Commands,
    added_wicket_query: Query<(Entity, &Transform), WithAddedWicket>,
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
