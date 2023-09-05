use bevy::prelude::{
    in_state, App, Commands, IntoSystemSetConfig, Local, OnEnter, Plugin, Res, States, SystemSet,
    Update,
};

use cricket_pong_controls::Controller;
use cricket_pong_game::{
    base::components::{
        instance::GameLobby,
        player::{PlayerOne, PlayerTwo},
    },
    Actions, GameInstance, GameplayPlugin,
};

fn spawn_local_game(mut commands: Commands) {
    let instance = GameInstance::new(0);
    commands.spawn((
        PlayerOne,
        PlayerOne::name(),
        Controller::One,
        instance.clone(),
    ));
    commands.spawn((
        PlayerTwo,
        PlayerTwo::name(),
        Controller::Two,
        instance.clone(),
    ));
    commands.spawn((GameLobby::default(), GameLobby::name(), instance));
}

fn yield_local_ticks(actions: Res<Actions>, mut tick: Local<u16>) -> Vec<(u16, Actions)> {
    let result = (*tick, actions.clone());
    *tick += 1;
    vec![result]
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, SystemSet)]
pub struct LocalGameplaySet;

pub struct LocalGameplayPlugin<State: States> {
    active_screen: State,
}

impl<State> LocalGameplayPlugin<State>
where
    State: States,
{
    pub fn new(active_screen: State) -> Self {
        LocalGameplayPlugin { active_screen }
    }
}

impl<State> Plugin for LocalGameplayPlugin<State>
where
    State: States + Copy,
{
    fn build(&self, app: &mut App) {
        app.configure_set(
            Update,
            LocalGameplaySet.run_if(in_state(self.active_screen)),
        )
        .add_plugins(GameplayPlugin::new(LocalGameplaySet, yield_local_ticks))
        .add_systems(OnEnter(self.active_screen), spawn_local_game);
    }
}
