use bevy::prelude::{App, IntoSystemConfigs, Plugin};

use big_brain::{prelude::BigBrainPlugin, BigBrainSet};

use cricket_pong_game::GameplayMarkerPlugin;

mod batter;
pub use batter::BatterBotBundle;

pub struct BotControllerPlugin;

impl Plugin for BotControllerPlugin {
    fn build(&self, app: &mut App) {
        assert!(GameplayMarkerPlugin::is_added(app));
        app.add_plugin(BigBrainPlugin)
            .add_systems(
                (batter::swing_cw_scorer, batter::swing_ccw_scorer).in_set(BigBrainSet::Scorers),
            )
            .add_systems(
                (batter::swing_cw_action, batter::swing_ccw_action).in_set(BigBrainSet::Actions),
            );
    }
}
