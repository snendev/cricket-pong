use bevy_ecs::prelude::{Added, Commands, Entity, Or, Query, ResMut, With, Without};
use bevy_log::{debug, info};

use cricket_pong_base::{
    actions::Actions,
    components::{
        ball::{Ball, BallBundle},
        batter::{Batter, BatterBundle},
        boundary::{Boundary, BoundaryBundle},
        fielder::{Fielder, FielderBundle, FielderPosition, FielderRing, FielderTrackBundle},
        phase::GamePhase,
        physics::{ExternalImpulse, Transform, Velocity},
        player::{PlayerOne, PlayerTwo, Position},
        scoreboard::ScoreboardBundle,
        wicket::{Wicket, WicketBundle},
    },
    lobby::components::{GameInstance, GameLobby},
};

use crate::{
    objects::{
        BallPhysicsBundle, BatterPhysicsBundle, BoundaryPhysicsBundle, FielderPhysicsBundle,
        WicketPhysicsBundle,
    },
    ShouldTick,
};

// NOTE: only spawns scenes for games that are hosted by this app
// (via ShouldTick on the GameLobby and Player components)
pub(crate) fn spawn_scene(
    mut commands: Commands,
    mut added_games_query: Query<
        (Entity, &mut GameLobby, &GameInstance),
        (
            With<ShouldTick>,
            Without<GamePhase>,
            Or<(Added<GameLobby>, Added<ShouldTick>)>,
        ),
    >,
    player_one_query: Query<(Entity, &GameInstance), (With<PlayerOne>, With<ShouldTick>)>,
    player_two_query: Query<(Entity, &GameInstance), (With<PlayerTwo>, With<ShouldTick>)>,
) -> Vec<(GameInstance, Vec<Entity>)> {
    let mut entities = Vec::new();
    for (lobby_entity, mut lobby, instance) in added_games_query.iter_mut() {
        let mut lobby_entities = Vec::new();

        debug!("Spawning game entities for instance {}", instance);
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

        entities.push((instance.clone(), lobby_entities));

        // insert player positions
        for (entity, player_instance) in player_one_query.iter() {
            if player_instance == instance {
                info!("Player 1 found! ({:?})", entity);
                commands.entity(entity).insert(Position::batter());
            }
        }
        for (entity, player_instance) in player_two_query.iter() {
            if player_instance == instance {
                info!("Player 2 found! ({:?})", entity);
                commands.entity(entity).insert(Position::fielder());
            }
        }

        lobby.activate();
        commands.entity(lobby_entity).insert(GamePhase::default());
    }

    entities
}

pub(crate) fn attach_ball_physics_components(
    mut commands: Commands,
    added_ball_query: Query<
        (Entity, &Transform, &Velocity, &ExternalImpulse),
        (Added<Ball>, With<ShouldTick>),
    >,
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
    added_fielder_query: Query<
        (Entity, &Fielder, &Transform, &Velocity),
        (Added<Fielder>, With<ShouldTick>),
    >,
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
    added_batter_query: Query<(Entity, &Transform, &Velocity), (Added<Batter>, With<ShouldTick>)>,
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
    added_boundary_query: Query<(Entity, &Transform), (Added<Boundary>, With<ShouldTick>)>,
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
    added_wicket_query: Query<(Entity, &Transform), (Added<Wicket>, With<ShouldTick>)>,
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
