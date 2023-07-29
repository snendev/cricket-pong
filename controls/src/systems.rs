use bevy::prelude::{debug, Added, Changed, Commands, Entity, Or, Query, ResMut};

use leafwing_input_manager::{prelude::ActionState, InputManagerBundle};

use cricket_pong_game::base::{
    actions::{Action, Actions},
    components::player::Position,
};

use crate::{
    BatterControl, BatterControllerBundle, BatterControllerBundle2, Controller, FielderControl,
    FielderControllerBundle, FielderControllerBundle2,
};

pub(crate) fn queue_inputs(
    batter_query: Query<(Entity, &ActionState<BatterControl>)>,
    fielder_query: Query<(Entity, &ActionState<FielderControl>)>,
    mut actions: ResMut<Actions>,
) {
    for (entity, action_state) in batter_query.iter() {
        let batter_actions = action_state.get_pressed();
        actions.0.extend(
            batter_actions
                .into_iter()
                .map(|action| (entity, Action::Batter(action.into()))),
        );
    }
    for (entity, action_state) in fielder_query.iter() {
        let fielder_actions = action_state.get_pressed();
        actions.0.extend(
            fielder_actions
                .into_iter()
                .map(|action| (entity, Action::Fielder(action.into()))),
        );
    }
}

pub(crate) fn sync_controllers(
    mut commands: Commands,
    player_query: Query<
        (Entity, &Position, &Controller),
        Or<(Changed<Position>, Added<Controller>)>,
    >,
) {
    for (entity, position, controller) in player_query.iter() {
        debug!(
            "Attaching controller {} position {} to entity ({:?})",
            controller, position, entity
        );
        let mut builder = commands.entity(entity);
        match (position, controller) {
            (Position::Fielder, Controller::One) => {
                builder
                    .remove::<InputManagerBundle<BatterControl>>()
                    .insert(FielderControllerBundle::new());
            }
            (Position::Batter, Controller::One) => {
                builder
                    .remove::<InputManagerBundle<FielderControl>>()
                    .insert(BatterControllerBundle::new());
            }
            (Position::Fielder, Controller::Two) => {
                builder
                    .remove::<InputManagerBundle<BatterControl>>()
                    .insert(FielderControllerBundle2::new());
            }
            (Position::Batter, Controller::Two) => {
                builder
                    .remove::<InputManagerBundle<FielderControl>>()
                    .insert(BatterControllerBundle2::new());
            }
        };
    }
}
