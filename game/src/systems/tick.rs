use bevy_ecs::prelude::{Commands, Entity, NextState, Query, Res, ResMut, State, With, Without};
use bevy_hierarchy::prelude::BuildChildren;
use bevy_math::prelude::Vec2;
use bevy_time::prelude::Time;
use bevy_transform::prelude::{GlobalTransform, Transform};

use bevy_rapier2d::prelude::{ExternalImpulse, Velocity};

use cricket_pong_base::{
    ball::Ball,
    batter::Batter,
    fielder::{Fielder, FielderPosition, FielderRing},
};

use crate::{
    actions::{Action, Actions, BatterAction, FielderAction},
    GamePhase,
};

pub(crate) fn ready_bowling_phase(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &mut Transform, &mut Velocity), With<Ball>>,
    fielders_query: Query<(Entity, &Fielder)>,
    mut state: ResMut<NextState<GamePhase>>,
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
        state.set(GamePhase::Bowling);
    }
}

pub(crate) fn consume_actions(
    mut commands: Commands,
    mut actions: ResMut<Actions>,
    state: Res<State<GamePhase>>,
    mut next_state: ResMut<NextState<GamePhase>>,
    mut fielders_query: Query<(&Fielder, &mut Velocity)>,
    mut batter_query: Query<(&mut Batter, &mut Velocity), Without<Fielder>>,
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
            Action::Fielder(FielderAction::Bowl) => {
                if *state != GamePhase::Bowling || next_state.0.is_some() {
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
                impulse.impulse += direction_vector * Fielder::BOWL_IMPULSE;
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
                    if fielder.ring == ring_to_match {
                        velocity.angvel = rotation_direction * Fielder::ROTATION_SPEED;
                    }
                }
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
