use bevy::prelude::{
    Added, App, Entity, Name, NextState, Or, Query, ResMut, Startup, States, SystemSet, Update,
};

use bevy_geppetto::Test;

use cricket_pong_controls::PlayerControllerPlugin;
use cricket_pong_graphics::GraphicsPlugin;

use cricket_pong_app_lib::{
    networking::{
        self,
        components::{PredictionOf, SourceOf},
    },
    AppScreen,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, SystemSet)]
pub struct GameplaySet;
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum TestState {
    #[default]
    Test,
    Complete,
}

fn set_state(mut state: ResMut<NextState<AppScreen>>) {
    state.set(AppScreen::OnlineGame);
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
                .add_state::<AppScreen>()
                .add_plugins(networking::OnlineGameplayPlugin)
                .register_type::<networking::components::SourceOf>()
                .register_type::<networking::components::PredictionOf>()
                .add_plugins(GraphicsPlugin::new(TestState::Complete))
                .add_plugins(PlayerControllerPlugin)
                .add_systems(Startup, set_state)
                .add_systems(Update, hydrate_names);
        },
    }
    .run();
}
