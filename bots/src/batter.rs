use bevy::prelude::{Bundle, Component, GlobalTransform, Quat, Query, ResMut, Vec2, With};

use big_brain::{
    prelude::{ActionBuilder, ActionState, Highest, ScorerBuilder, Thinker, ThinkerBuilder},
    scorers::Score,
    thinker::Actor,
};

use cricket_pong_game::{
    actions::{Action, Actions, BatterAction},
    base::{
        ball::Ball,
        batter::{Bat, Batter},
    },
};

const _IDEAL_SWING_ANGLE: f32 = std::f32::consts::FRAC_PI_4;
const STRIKE_ZONE_MIN: f32 = Batter::RADIUS;
const STRIKE_ZONE_MAX: f32 = Batter::RADIUS + Bat::HWIDTH * 2.;

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub(crate) struct SwingCWScorer;
#[derive(Clone, Component, Debug, ScorerBuilder)]
pub(crate) struct SwingCCWScorer;
#[derive(Clone, Component, Debug, ScorerBuilder)]
pub(crate) struct MoveCWScorer;
#[derive(Clone, Component, Debug, ScorerBuilder)]
pub(crate) struct MoveCCWScorer;

#[derive(Clone, Component, Debug, ActionBuilder)]
pub(crate) struct PerformSwingCW;
#[derive(Clone, Component, Debug, ActionBuilder)]
pub(crate) struct PerformSwingCCW;
#[derive(Clone, Component, Debug, ActionBuilder)]
pub(crate) struct PerformMoveCW;
#[derive(Clone, Component, Debug, ActionBuilder)]
pub(crate) struct PerformMoveCCW;

#[derive(Bundle)]
pub struct BatterBotBundle {
    thinker: ThinkerBuilder,
}

impl BatterBotBundle {
    pub fn new() -> Self {
        BatterBotBundle {
            thinker: Thinker::build()
                .label("Batter Bot")
                .picker(Highest)
                .when(SwingCWScorer, PerformSwingCW)
                .when(SwingCCWScorer, PerformSwingCCW)
                .when(MoveCWScorer, PerformMoveCW)
                .when(MoveCCWScorer, PerformMoveCCW),
        }
    }
}

pub(crate) enum IsInRange {
    Yes,
    No,
}

pub(crate) enum Direction {
    Clockwise,
    CounterClockwise,
}

pub(crate) fn calculate_swing_data(
    ball_position: Vec2,
    batter_rotation: Quat,
) -> (IsInRange, Direction) {
    let bat_angle = batter_rotation
        .angle_between(Quat::IDENTITY)
        // bat is angled 1/4 circle from the batter stance
        + std::f32::consts::FRAC_PI_2;
    let ball_origin_distance = ball_position.length();
    let is_in_range =
        ball_origin_distance > STRIKE_ZONE_MIN && ball_origin_distance < STRIKE_ZONE_MAX;
    let is_in_range = if is_in_range {
        IsInRange::Yes
    } else {
        IsInRange::No
    };

    let ball_angle = ball_position.angle_between(Vec2::Y);
    let delta_angle = bat_angle - ball_angle;
    let is_ccw_direction = delta_angle.is_sign_positive() && delta_angle < std::f32::consts::PI;
    let direction = if is_ccw_direction {
        Direction::CounterClockwise
    } else {
        Direction::Clockwise
    };
    (is_in_range, direction)
}

pub(crate) fn swing_cw_scorer(
    ball_position_query: Query<&GlobalTransform, With<Ball>>,
    batter_position_query: Query<&GlobalTransform, With<Batter>>,
    mut scorer_query: Query<(&Actor, &mut Score), With<SwingCWScorer>>,
) {
    let Ok(ball_transform) = ball_position_query.get_single() else { return };
    let ball_position = Vec2::new(
        ball_transform.translation().x,
        ball_transform.translation().y,
    );
    let Ok(batter_transform) = batter_position_query.get_single() else { return };
    let batter_rotation = batter_transform.to_scale_rotation_translation().1;
    for (Actor(_actor), mut score) in scorer_query.iter_mut() {
        // if ball in strike_zone
        match calculate_swing_data(ball_position, batter_rotation) {
            (IsInRange::Yes, Direction::Clockwise) => {
                score.set(1.);
            }
            _ => {
                score.set(0.);
            }
        }
    }
}

pub(crate) fn swing_ccw_scorer(
    ball_position_query: Query<&GlobalTransform, With<Ball>>,
    batter_position_query: Query<&GlobalTransform, With<Batter>>,
    mut scorer_query: Query<(&Actor, &mut Score), With<SwingCCWScorer>>,
) {
    let Ok(ball_transform) = ball_position_query.get_single() else { return };
    let ball_position = Vec2::new(
        ball_transform.translation().x,
        ball_transform.translation().y,
    );
    let Ok(batter_transform) = batter_position_query.get_single() else { return };
    let batter_rotation = batter_transform.to_scale_rotation_translation().1;
    for (Actor(_actor), mut score) in scorer_query.iter_mut() {
        // if ball in strike_zone
        match calculate_swing_data(ball_position, batter_rotation) {
            (IsInRange::Yes, Direction::CounterClockwise) => {
                score.set(1.);
            }
            _ => {
                score.set(0.);
            }
        }
    }
}

pub(crate) fn swing_cw_action(
    mut actions: ResMut<Actions>,
    mut bot_action_query: Query<(&Actor, &mut ActionState), With<PerformSwingCW>>,
) {
    for (Actor(_actor), mut action_state) in bot_action_query.iter_mut() {
        match *action_state {
            ActionState::Requested => {
                actions.0.push(Action::Batter(BatterAction::SwingCW));
                *action_state = ActionState::Success;
            }
            _ => {}
        }
    }
}

pub(crate) fn swing_ccw_action(
    mut actions: ResMut<Actions>,
    mut bot_action_query: Query<(&Actor, &mut ActionState), With<PerformSwingCCW>>,
) {
    for (Actor(_actor), mut action_state) in bot_action_query.iter_mut() {
        match *action_state {
            ActionState::Requested => {
                actions.0.push(Action::Batter(BatterAction::SwingCCW));
                *action_state = ActionState::Success;
            }
            _ => {}
        }
    }
}
