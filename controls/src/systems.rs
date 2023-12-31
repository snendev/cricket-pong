use bevy::prelude::{Changed, Commands, Entity, Query, ResMut, With, Without};

use leafwing_input_manager::{prelude::ActionState, InputManagerBundle};

use cricket_pong_game::{
    actions::{Action, Actions},
    base::{PlayerOne, PlayerTwo, Position},
};

use crate::{
    BatterControl, BatterControllerBundle, BatterControllerBundle2, FielderControl,
    FielderControllerBundle, FielderControllerBundle2,
};

pub(crate) fn queue_inputs(
    batter_query: Query<&ActionState<BatterControl>>,
    fielder_query: Query<&ActionState<FielderControl>>,
    mut actions: ResMut<Actions>,
) {
    for action_state in batter_query.iter() {
        let batter_actions = action_state.get_pressed();
        actions.0.extend(
            batter_actions
                .into_iter()
                .map(|action| Action::Batter(action.into())),
        );
    }
    for action_state in fielder_query.iter() {
        let fielder_actions = action_state.get_pressed();
        actions.0.extend(
            fielder_actions
                .into_iter()
                .map(|action| Action::Fielder(action.into())),
        );
    }
}

pub(crate) fn sync_controllers(
    mut commands: Commands,
    player_one_query: Query<
        (Entity, &Position),
        (With<PlayerOne>, Without<PlayerTwo>, Changed<Position>),
    >,
    player_two_query: Query<
        (Entity, &Position),
        (With<PlayerTwo>, Without<PlayerOne>, Changed<Position>),
    >,
) {
    for (entity, position) in player_one_query.iter() {
        let mut builder = commands.entity(entity);
        match *position {
            Position::Fielder => {
                builder
                    .remove::<InputManagerBundle<BatterControl>>()
                    .insert(FielderControllerBundle::new());
            }
            Position::Batter => {
                builder
                    .remove::<InputManagerBundle<FielderControl>>()
                    .insert(BatterControllerBundle::new());
            }
        };
    }
    for (entity, position) in player_two_query.iter() {
        let mut builder = commands.entity(entity);
        match *position {
            Position::Fielder => {
                builder
                    .remove::<InputManagerBundle<BatterControl>>()
                    .insert(FielderControllerBundle2::new());
            }
            Position::Batter => {
                builder
                    .remove::<InputManagerBundle<FielderControl>>()
                    .insert(BatterControllerBundle2::new());
            }
        };
    }
}
