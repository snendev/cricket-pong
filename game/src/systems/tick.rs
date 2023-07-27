use bevy_ecs::{
    prelude::{NextState, Query, Res, ResMut, State, With, Without},
    schedule::SystemSet,
};
use bevy_math::prelude::{Vec2 as BevyVec2, Vec3 as BevyVec3};
use bevy_time::prelude::Time;
use bevy_transform::prelude::Transform as BevyTransform;

use bevy_rapier2d::prelude::Velocity as RapierVelocity;

use cricket_pong_base::{
    actions::{Action, Actions, BatterAction, FielderAction},
    components::{
        ball::Ball,
        batter::Batter,
        fielder::{Fielder, FielderPosition, FielderRing},
        physics::{ExternalImpulse, Transform, Vec2, Velocity},
    },
};

use crate::GamePhase;

pub(crate) fn track_bowler_transform(
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    fielders_query: Query<(&Transform, &Fielder), Without<Ball>>,
) {
    let Ok((mut transform, mut velocity)) = ball_query.get_single_mut() else { return };
    let Some(fielder_transform) = fielders_query.iter().find_map(|(transform, fielder)| {
        if *fielder.position == FielderPosition::Top && *fielder.ring == FielderRing::Infield {
            Some(transform)
        } else {
            None
        }
    }) else { return };

    // enforce that the ball is not moving
    *velocity = Velocity::from(&RapierVelocity::zero());
    // track the bowler paddle
    let fielder_translation = BevyVec3::from(&*fielder_transform.translation);
    let target_translation =
        fielder_translation - fielder_translation.normalize() * (Fielder::HDEPTH + Ball::RADIUS);
    *transform = Transform::from(&BevyTransform::from_translation(target_translation));
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct ActionsSet;

pub(crate) fn consume_actions(
    mut actions: ResMut<Actions>,
    state: Res<State<GamePhase>>,
    mut next_state: ResMut<NextState<GamePhase>>,
    mut fielders_query: Query<(&Fielder, &mut Velocity)>,
    mut batter_query: Query<(&mut Batter, &mut Velocity), Without<Fielder>>,
    mut ball_query: Query<(&mut ExternalImpulse, &Transform), With<Ball>>,
    time: Res<Time>,
) {
    for (_, mut velocity) in fielders_query.iter_mut() {
        *velocity = Velocity::from(&RapierVelocity::zero());
    }
    if let Ok((mut bat, mut velocity)) = batter_query.get_single_mut() {
        if let Some(swing_timer) = bat.timer.as_mut() {
            if *swing_timer <= 0. {
                *bat.timer = None;
                *velocity.angular = 0.;
            } else {
                *swing_timer -= time.delta_seconds();
            }
        } else {
            *velocity.angular = 0.;
        }
    }
    for (_entity, action) in actions.0.drain(..) {
        match action {
            Action::Fielder(FielderAction::Bowl) => {
                if *state != GamePhase::Bowling || next_state.0.is_some() {
                    continue;
                };
                let Ok((mut impulse, transform)) = ball_query.get_single_mut() else { continue };
                let direction_vector =
                    (-BevyVec2::new(transform.translation.x, transform.translation.y)).normalize();
                *impulse.linear = Vec2::from(
                    &(BevyVec2::from(&*impulse.linear) + direction_vector * Fielder::BOWL_IMPULSE),
                );
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
                for (fielder, mut velocity) in fielders_query.iter_mut() {
                    if *fielder.ring == ring_to_match {
                        *velocity.angular = rotation_direction * Fielder::ROTATION_SPEED;
                    }
                }
            }
            Action::Batter(movement) => {
                if let Ok((mut bat, mut velocity)) = batter_query.get_single_mut() {
                    if bat.timer.is_none() {
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
                        *velocity.angular = angular_velocity;
                    }
                }
            }
        }
    }
}
