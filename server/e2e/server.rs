use bevy_app::prelude::{App, PostUpdate};
use bevy_ecs::prelude::{Added, Commands, Entity, Query, States};

use bevy_geppetto::Test;

use cricket_pong_game::ShouldTick;
use cricket_pong_graphics::{GraphicsPlugin, ShouldRender};
use cricket_pong_server::ServerPlugin;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum TestState {
    #[default]
    Test,
    Complete,
}

fn render_graphics(mut commands: Commands, entity_query: Query<Entity, Added<ShouldTick>>) {
    for entity in entity_query.iter() {
        commands.entity(entity).insert(ShouldRender);
    }
}

fn main() {
    Test {
        label: "Game sandbox".to_string(),
        setup: |app: &mut App| {
            app.add_state::<TestState>()
                .add_plugins(ServerPlugin)
                .add_plugins(GraphicsPlugin::new(TestState::Complete))
                .add_systems(PostUpdate, render_graphics);
        },
    }
    .run();
}
