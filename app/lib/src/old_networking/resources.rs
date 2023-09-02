use bevy::prelude::Resource;

use cricket_pong_game::base::actions::Actions;

#[derive(Resource, Default)]
pub struct TickHistory(pub CommandHistory<Actions>);
