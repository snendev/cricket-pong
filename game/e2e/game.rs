use bevy_app::{
    prelude::{App, Startup},
    PostUpdate,
};
use bevy_ecs::{
    prelude::{Commands, Entity, Local, OnExit, Query, Res, States, SystemSet},
    query::Added,
    system::In,
};

use bevy_rapier2d::render::RapierDebugRenderPlugin;

use bevy_geppetto::Test;

use cricket_pong_base::{
    components::player::{PlayerOne, PlayerTwo},
    lobby::components::{GameInstance, GameLobby},
};
use cricket_pong_controls::{Controller, PlayerControllerPlugin};
use cricket_pong_graphics::{GraphicsPlugin, ShouldRender};

use cricket_pong_game::{Actions, GameplayPlugin, ShouldTick};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, SystemSet)]
pub struct GameplaySet;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum TestState {
    #[default]
    Test,
    Complete,
}

fn spawn_lobby(mut commands: Commands) {
    commands.spawn((
        GameLobby::default(),
        GameLobby::name(),
        GameInstance::new(0),
        ShouldTick,
        ShouldRender,
    ));
    commands.spawn((
        PlayerOne,
        PlayerOne::name(),
        GameInstance::new(0),
        Controller::One,
        ShouldTick,
        ShouldRender,
    ));
    commands.spawn((
        PlayerTwo,
        PlayerTwo::name(),
        GameInstance::new(0),
        Controller::Two,
        ShouldTick,
        ShouldRender,
    ));
}

fn yield_local_ticks(actions: Res<Actions>, mut tick: Local<u16>) -> Vec<(u16, Actions)> {
    let result = (*tick, actions.clone());
    *tick += 1;
    vec![result]
}

fn unload_lobby(mut lobby_query: Query<&mut GameLobby>) {
    for mut lobby in lobby_query.iter_mut() {
        lobby.unload();
    }
}

fn render_graphics(mut commands: Commands, entity_query: Query<Entity, Added<ShouldTick>>) {
    for entity in entity_query.iter() {
        commands.entity(entity).insert(ShouldRender);
    }
}

fn noop(_: In<Vec<(GameInstance, Vec<Entity>)>>) {}

fn main() {
    Test {
        label: "Game sandbox".to_string(),
        setup: |app: &mut App| {
            app.add_state::<TestState>()
                .add_plugins((
                    RapierDebugRenderPlugin::default(),
                    GameplayPlugin::new(GameplaySet, yield_local_ticks, noop),
                ))
                .add_plugins((
                    GraphicsPlugin::new(TestState::Complete),
                    PlayerControllerPlugin,
                ))
                .add_systems(Startup, spawn_lobby)
                .add_systems(PostUpdate, render_graphics)
                .add_systems(OnExit(TestState::Test), unload_lobby);
        },
    }
    .run();
}
