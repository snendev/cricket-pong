use bevy_ecs::{
    prelude::{
        Commands, Entity, EventReader, NextState, Query, Res, ResMut, State, States, With, Without,
    },
    system::Local,
};
use bevy_hierarchy::prelude::BuildChildren;
use bevy_log::prelude::info;
use bevy_math::prelude::Vec2;
use bevy_time::prelude::Time;
use bevy_transform::prelude::{GlobalTransform, Transform};

use bevy_rapier2d::{
    prelude::{CollisionEvent, ExternalImpulse, Velocity},
    rapier::prelude::CollisionEventFlags,
};

use cricket_pong_base::{
    ball::Ball,
    batter::{Batter, Wicket},
    fielder::{Boundary, Fielder, FielderPosition, FielderRing},
    Objective, Player,
};

use crate::{
    actions::{Action, Actions, BatterAction, FielderAction},
    objects::{ball::BallBundle, batter::BatterSpawner, field::FieldersSpawner},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
pub(crate) enum GamePhase {
    #[default]
    Inactive,
    Pitching,
    Active,
}

pub(crate) fn spawn_scene(mut commands: Commands, mut state: ResMut<NextState<GamePhase>>) {
    info!("Startup game");
    commands.spawn(BallBundle::new(Transform::from_xyz(0., 0., 1.)));
    FieldersSpawner::spawn(&mut commands);
    BatterSpawner::spawn(&mut commands);
    state.set(GamePhase::Pitching);
}

pub(crate) fn ready_pitching_phase(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &mut Transform, &mut Velocity), With<Ball>>,
    fielders_query: Query<(Entity, &Fielder)>,
) {
    let Ok((ball, mut transform, mut velocity)) = ball_query.get_single_mut() else { return };
    if let Some(fielder) = fielders_query.iter().find_map(|(entity, fielder)| {
        if fielder.position == FielderPosition::Top && fielder.ring == FielderRing::Infield {
            Some(entity)
        } else {
            None
        }
    }) {
        commands.entity(fielder).add_child(ball);
        transform.translation.x = 0.;
        transform.translation.y = -(Fielder::HDEPTH + Ball::RADIUS);
        *velocity = Velocity::zero();
    }
}

pub(crate) fn consume_actions(
    mut commands: Commands,
    mut actions: ResMut<Actions>,
    state: Res<State<GamePhase>>,
    mut next_state: ResMut<NextState<GamePhase>>,
    mut fielders_query: Query<(&FielderRing, &mut Velocity)>,
    mut batter_query: Query<(&mut Batter, &mut Velocity), Without<FielderRing>>,
    mut ball_query: Query<
        (
            Entity,
            &mut ExternalImpulse,
            &mut Transform,
            &GlobalTransform,
        ),
        With<Ball>,
    >,
    time: Res<Time>,
) {
    for (_, mut velocity) in fielders_query.iter_mut() {
        *velocity = Velocity::zero();
    }
    if let Ok((mut bat, mut velocity)) = batter_query.get_single_mut() {
        if let Some(swing_timer) = bat.swing_timer.as_mut() {
            if *swing_timer <= 0. {
                bat.swing_timer = None;
                velocity.angvel = 0.;
            } else {
                *swing_timer -= time.delta_seconds();
            }
        } else {
            velocity.angvel = 0.;
        }
    }
    for action in actions.0.drain(..) {
        match action {
            Action::Fielder(FielderAction::Pitch) => {
                if *state != GamePhase::Pitching || next_state.0.is_some() {
                    continue;
                };
                let Ok((ball, mut impulse, mut transform, global_transform)) = ball_query.get_single_mut() else { continue };
                let origin = Vec2::new(
                    global_transform.translation().x,
                    global_transform.translation().y,
                );
                transform.translation.x = origin.x;
                transform.translation.y = origin.y;
                commands.entity(ball).remove_parent();
                let direction_vector = (-origin).normalize();
                impulse.impulse += direction_vector * Fielder::PITCH_IMPULSE;
                next_state.set(GamePhase::Active);
            }
            Action::Fielder(movement) => {
                let ring_to_match = match movement {
                    FielderAction::MoveInfieldCW | FielderAction::MoveInfieldCCW => {
                        FielderRing::Infield
                    }
                    FielderAction::MoveOutfieldCW | FielderAction::MoveOutfieldCCW => {
                        FielderRing::Outfield
                    }
                    _ => continue,
                };
                let Some(rotation_direction) = movement.rotation_direction() else { continue };
                let Some(mut velocity) = fielders_query
                    .iter_mut()
                    .find_map(|(ring, velocity)| {
                        if *ring == ring_to_match {
                            Some(velocity)
                        } else {
                            None
                        }
                    }) else { continue };
                velocity.angvel = rotation_direction * Fielder::ROTATION_SPEED;
            }
            Action::Batter(movement) => {
                if let Ok((mut bat, mut velocity)) = batter_query.get_single_mut() {
                    if bat.swing_timer.is_none() {
                        let angular_velocity = movement.rotation_direction()
                            * match movement {
                                BatterAction::MoveCW | BatterAction::MoveCCW => {
                                    Batter::ROTATION_SPEED
                                }
                                BatterAction::SwingCW | BatterAction::SwingCCW => {
                                    Batter::SWING_VELOCITY
                                }
                            };
                        bat.swing_timer = Some(Batter::SWING_TIME);
                        velocity.angvel = angular_velocity;
                    }
                }
            }
        }
    }
}

pub(crate) fn register_goals(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<&mut Player>,
    ball_query: Query<&Ball>,
    wicket_query: Query<&Wicket>,
    boundary_query: Query<&Boundary>,
    fielder_query: Query<&Fielder>,
    mut state: ResMut<NextState<GamePhase>>,
    mut pass_count: Local<u8>,
) {
    for event in collision_events.iter() {
        // score 1 for batter if the ball goes outside the boundary
        if let CollisionEvent::Stopped(entity1, entity2, flags) = event {
            if flags.contains(CollisionEventFlags::REMOVED) {
                continue;
            };
            let other_entity = if ball_query.contains(*entity1) {
                *entity2
            } else if ball_query.contains(*entity2) {
                *entity1
            } else {
                continue;
            };
            if boundary_query.contains(other_entity) {
                for mut player in player_query.iter_mut() {
                    if player.objective == Objective::Batting {
                        player.score += 1;
                        info!("Score for batter: {}!", player.score);
                    }
                }
                *pass_count = 0;
                state.set(GamePhase::Pitching);
            }
        }
        if let CollisionEvent::Started(entity1, entity2, _flags) = event {
            let other_entity = if ball_query.contains(*entity1) {
                *entity2
            } else if ball_query.contains(*entity2) {
                *entity1
            } else {
                continue;
            };
            // score 3 for fielder if the ball hits the wicket
            if wicket_query.contains(other_entity) {
                for mut player in player_query.iter_mut() {
                    if player.objective == Objective::Fielding {
                        player.score += 3;
                        info!("Score for fielder: {}!", player.score);
                    }
                }
                *pass_count = 0;
                state.set(GamePhase::Pitching);
            }
            // score 1 for fielder if the ball is passed between paddles 4 times
            if fielder_query.contains(other_entity) {
                *pass_count += 1;
                if *pass_count >= 4 {
                    for mut player in player_query.iter_mut() {
                        if player.objective == Objective::Fielding {
                            player.score += 1;
                            info!("Score for fielder: {}!", player.score);
                        }
                    }
                    *pass_count = 0;
                    state.set(GamePhase::Pitching);
                }
            }
        }
    }
}
