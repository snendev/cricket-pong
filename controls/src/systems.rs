use bevy::prelude::{Changed, Commands, Entity, Query, ResMut};

use leafwing_input_manager::{prelude::ActionState, InputManagerBundle};

use cricket_pong_game::{
    actions::{Action, Actions},
    base::{Identity, Player, Position},
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
    players_query: Query<(Entity, &Player, &Position), Changed<Position>>,
) {
    for (entity, player, position) in players_query.iter() {
        let mut builder = commands.entity(entity);
        match (*position, player.id) {
            (Position::Fielder, Identity::One) => {
                builder
                    .remove::<InputManagerBundle<BatterControl>>()
                    .insert(FielderControllerBundle::new());
            }
            (Position::Fielder, Identity::Two) => {
                builder
                    .remove::<InputManagerBundle<BatterControl>>()
                    .insert(FielderControllerBundle2::new());
            }
            (Position::Batter, Identity::One) => {
                builder
                    .remove::<InputManagerBundle<FielderControl>>()
                    .insert(BatterControllerBundle::new());
            }
            (Position::Batter, Identity::Two) => {
                builder
                    .remove::<InputManagerBundle<FielderControl>>()
                    .insert(BatterControllerBundle2::new());
            }
        };
    }
}
