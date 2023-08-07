use bevy_ecs::{
    prelude::{Query, Res, ResMut, SystemSet, With, Without},
    query::Or,
};
use bevy_log::debug;
use bevy_math::prelude::Vec2;
use bevy_time::prelude::Time;
use bevy_transform::prelude::Transform;

use cricket_pong_base::{
    actions::{Action, Actions, BatterAction, FielderAction},
    components::{
        ball::Ball,
        batter::Batter,
        fielder::{Fielder, FielderPosition, FielderRing},
        phase::GamePhase,
        player::{PlayerOne, PlayerTwo},
    },
    lobby::components::GameInstance,
    rapier::prelude::{ExternalImpulse, Velocity},
};

use crate::ShouldTick;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct ActionsSet;

type WithBall = (With<Ball>, With<ShouldTick>);
type WithoutBall = (Without<Ball>, With<ShouldTick>);

pub(crate) fn track_bowler_transform(
    games_query: Query<(&GameInstance, &GamePhase), With<ShouldTick>>,
    mut balls_query: Query<(&GameInstance, &mut Transform, &mut Velocity), WithBall>,
    fielders_query: Query<(&GameInstance, &Transform, &Fielder), WithoutBall>,
) {
    for (game_instance, phase) in games_query.iter() {
        if !phase.is_bowling() {
            continue;
        };

        let Some((mut transform, mut velocity)) = balls_query.iter_mut().find_map(|(instance, transform, velocity)| {
            if instance == game_instance {
                Some((transform, velocity))
            } else {
                None
            }
        }) else { continue };
        let Some(fielder_transform) = fielders_query.iter().find_map(|(instance, transform, fielder)| {
            if instance == game_instance && *fielder.position == FielderPosition::Top && *fielder.ring == FielderRing::Infield {
                Some(transform)
            } else {
                None
            }
        }) else { continue };

        // enforce that the ball is not moving
        *velocity = Velocity::zero();
        // track the bowler paddle
        let fielder_translation = fielder_transform.translation;
        let target_translation = fielder_translation
            - fielder_translation.normalize() * (Fielder::HDEPTH + Ball::RADIUS);
        *transform = Transform::from_translation(target_translation);
    }
}

type WithoutObjects = (
    Without<Fielder>,
    Without<Batter>,
    Without<Ball>,
    With<ShouldTick>,
);
type WithSomePlayer = (With<ShouldTick>, Or<(With<PlayerOne>, With<PlayerTwo>)>);
type WithoutBatterOrBall = (Without<Batter>, Without<Ball>, With<ShouldTick>);
type WithoutFielderOrBall = (Without<Fielder>, Without<Ball>, With<ShouldTick>);

pub(crate) fn consume_actions(
    mut actions: ResMut<Actions>,
    mut game_query: Query<(&GameInstance, &mut GamePhase), WithoutObjects>,
    player_query: Query<&GameInstance, WithSomePlayer>,
    mut fielders_query: Query<(&GameInstance, &Fielder, &mut Velocity), WithoutBatterOrBall>,
    mut batters_query: Query<(&GameInstance, &mut Batter, &mut Velocity), WithoutFielderOrBall>,
    mut ball_query: Query<(&GameInstance, &mut ExternalImpulse, &Transform), WithBall>,
    time: Res<Time>,
) {
    for (_, _, mut velocity) in fielders_query.iter_mut() {
        *velocity = Velocity::zero();
    }
    for (_, mut bat, mut velocity) in batters_query.iter_mut() {
        if let Some(swing_timer) = bat.timer.as_mut() {
            if *swing_timer <= 0. {
                *bat.timer = None;
                velocity.angvel = 0.;
            } else {
                *swing_timer -= time.delta_seconds();
            }
        } else {
            velocity.angvel = 0.;
        }
    }

    // TODO golly this is a lot of nested iteration
    for (entity, action) in actions.0.drain(..) {
        let Ok(game_instance) = player_query.get(entity) else { continue; };
        let Some(mut phase) =  game_query.iter_mut().find_map(|(instance, phase)| {
            if instance == game_instance {
                Some(phase)
            } else {
                None
            }
        }) else { continue; };

        debug!(
            "Processing action {:?} for entity ({:?}), instance {:?} in phase ({:?})",
            action, entity, game_instance, *phase
        );

        match action {
            Action::Fielder(FielderAction::Bowl) => {
                if !phase.is_bowling() {
                    continue;
                };
                let Some((mut impulse, transform)) = ball_query.iter_mut().find_map(|(instance, impulse, transform)| {
                    if instance == game_instance {
                        Some((impulse, transform))
                    } else {
                        None
                    }
                }) else { continue; };

                let direction_vector =
                    (-Vec2::new(transform.translation.x, transform.translation.y)).normalize();
                impulse.impulse += direction_vector * Fielder::BOWL_IMPULSE;
                phase.set_active();
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
                for (instance, fielder, mut velocity) in fielders_query.iter_mut() {
                    if instance == game_instance && *fielder.ring == ring_to_match {
                        velocity.angvel = rotation_direction * Fielder::ROTATION_SPEED;
                    }
                }
            }
            Action::Batter(movement) => {
                for (instance, mut bat, mut velocity) in batters_query.iter_mut() {
                    if instance == game_instance && bat.timer.is_none() {
                        let angular_velocity = movement.rotation_direction()
                            * match movement {
                                BatterAction::MoveCW | BatterAction::MoveCCW => {
                                    Batter::ROTATION_SPEED
                                }
                                BatterAction::SwingCW | BatterAction::SwingCCW => {
                                    Batter::SWING_VELOCITY
                                }
                            };
                        match movement {
                            BatterAction::SwingCW | BatterAction::SwingCCW => {
                                *bat.timer = Some(Batter::SWING_TIME);
                            }
                            _ => {}
                        };
                        velocity.angvel = angular_velocity;
                    }
                }
            }
        }
    }
}
