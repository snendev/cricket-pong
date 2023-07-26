use bevy::prelude::Resource;

use naia_bevy_client::CommandHistory;

use cricket_pong_game::base::actions::Actions;

#[derive(Resource, Default)]
pub struct TickHistory(pub CommandHistory<Actions>);
