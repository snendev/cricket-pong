use bevy::prelude::{Added, App, Entity, Name, Or, Query, States, Update};

use bevy_geppetto::Test;

use cricket_pong_controls::PlayerControllerPlugin;
use cricket_pong_graphics::GraphicsPlugin;

use cricket_pong_app_lib::networking::{
    components::{PredictionOf, SourceOf},
    OnlineGameplayPlugin,
};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum TestState {
    #[default]
    Test,
    Complete,
}

fn hydrate_names(
    mut query: Query<
        (Entity, &mut Name, Option<&PredictionOf>, Option<&SourceOf>),
        Or<(Added<PredictionOf>, Added<SourceOf>, Added<Name>)>,
    >,
) {
    for (entity, mut name, prediction, source) in query.iter_mut() {
        if let Some(PredictionOf(source)) = prediction {
            name.mutate(|name| {
                *name = format!("{} {:?} (Source: {:?})", name, entity, source);
            });
        } else if let Some(SourceOf(prediction)) = source {
            name.mutate(|name| {
                *name = format!("{} {:?} (Prediction: {:?})", name, entity, prediction);
            });
        }
    }
}

fn main() {
    Test {
        label: "Game sandbox".to_string(),
        setup: |app: &mut App| {
            app.add_state::<TestState>()
                .add_plugins(OnlineGameplayPlugin::new(TestState::Test))
                .register_type::<SourceOf>()
                .register_type::<PredictionOf>()
                .add_plugins(GraphicsPlugin::new(TestState::Complete))
                .add_plugins(PlayerControllerPlugin)
                .add_systems(Update, hydrate_names);
        },
    }
    .run();
}
