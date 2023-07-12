use bevy::prelude::{Query, ResMut};

use leafwing_input_manager::prelude::ActionState;

use cricket_pong_game::actions::{Action, Actions};

use crate::{BatterControl, FielderControl};

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
