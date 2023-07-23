use bevy::{
    prelude::{
        in_state, Added, AlignItems, App, BackgroundColor, BuildChildren, ButtonBundle, Changed,
        ChildBuilder, Color, Commands, Component, DespawnRecursiveExt, Display, Entity,
        FlexDirection, GridAutoFlow, IntoSystemConfigs, JustifyContent, NextState, NodeBundle,
        OnEnter, OnExit, Plugin, PositionType, PostUpdate, Query, Res, ResMut, States, Style,
        SystemSet, Text, TextBundle, TextStyle, UiRect, Val, With, Without,
    },
    ui::{BorderColor, GridPlacement, GridTrack, Interaction},
};

use cricket_pong_base::{Identity, Over, PlayerOne, PlayerTwo, Position, Score};

#[derive(Component)]
struct Scoreboard;

#[derive(Component)]
struct OverScoreboard;

#[derive(Component)]
struct GameoverPanel;

#[derive(Component)]
struct ScoreTracker {
    pub player: Identity,
    pub style: TextStyle,
}

#[derive(Component)]
struct PositionTracker {
    pub player: Identity,
    pub style: TextStyle,
}

#[derive(Component)]
struct BowlTracker {
    pub index: usize,
    pub style: TextStyle,
    pub parent: Entity,
}

fn spawn_player_scoreboard(
    commands: &mut Commands,
    score: &Score,
    position: &Position,
    player: Identity,
) {
    commands
        .spawn((
            Scoreboard,
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(12.)),
                    column_gap: Val::Px(4.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    left: match player {
                        Identity::One => Val::Percent(0.),
                        Identity::Two => Val::Auto,
                    },
                    right: match player {
                        Identity::One => Val::Auto,
                        Identity::Two => Val::Percent(0.),
                    },
                    ..Default::default()
                },
                background_color: BackgroundColor(match player {
                    Identity::One => Color::LIME_GREEN,
                    Identity::Two => Color::CYAN,
                }),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("Player {}", player),
                TextStyle {
                    color: Color::BLACK,
                    font_size: 16.,
                    ..Default::default()
                },
            ));
            let score_text_style = TextStyle {
                font_size: 28.,
                color: Color::BLACK,
                ..Default::default()
            };
            let position_text_style = TextStyle {
                color: Color::BLACK,
                ..Default::default()
            };
            parent.spawn((
                ScoreTracker {
                    player,
                    style: score_text_style.clone(),
                },
                TextBundle::from_section(score.0.to_string(), score_text_style),
            ));
            parent.spawn((
                PositionTracker {
                    player,
                    style: position_text_style.clone(),
                },
                TextBundle::from_section(position.to_string(), position_text_style),
            ));
        });
}

fn spawn_scoreboard(
    mut commands: Commands,
    player_one_query: Query<(&Score, &Position), (Added<PlayerOne>, Without<PlayerTwo>)>,
    player_two_query: Query<(&Score, &Position), (Added<PlayerTwo>, Without<PlayerOne>)>,
) {
    let Ok((score_one, position_one)) = player_one_query.get_single() else { return };
    let Ok((score_two, position_two)) = player_two_query.get_single() else { return };
    spawn_player_scoreboard(&mut commands, score_one, position_one, Identity::One);
    spawn_player_scoreboard(&mut commands, score_two, position_two, Identity::Two);
}

fn update_scoreboard(
    player_one_query: Query<(&Score, &Position), (With<PlayerOne>, Without<PlayerTwo>)>,
    player_two_query: Query<(&Score, &Position), (With<PlayerTwo>, Without<PlayerOne>)>,
    mut score_count_query: Query<(&mut Text, &ScoreTracker)>,
    mut position_tracker_query: Query<(&mut Text, &PositionTracker), Without<ScoreTracker>>,
) {
    let Ok((score_one, position_one)) = player_one_query.get_single() else { return };
    let Ok((score_two, position_two)) = player_two_query.get_single() else { return };

    for (score, position, identity) in vec![
        (score_one, position_one, Identity::One),
        (score_two, position_two, Identity::Two),
    ] {
        for (mut text, tracker) in score_count_query.iter_mut() {
            if tracker.player == identity {
                *text = Text::from_section(score.0.to_string(), tracker.style.clone());
            }
        }
        for (mut text, tracker) in position_tracker_query.iter_mut() {
            if tracker.player == identity {
                *text = Text::from_section(position.to_string(), tracker.style.clone());
            }
        }
    }
}

fn spawn_over_tracker(mut commands: Commands) {
    commands
        .spawn((
            OverScoreboard,
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    grid_auto_flow: GridAutoFlow::Column,
                    grid_template_rows: vec![GridTrack::min_content(), GridTrack::min_content()],
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.),
                    left: Val::Px(0.),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::NAVY),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            let text_style = TextStyle {
                font_size: 28.,
                color: Color::BLACK,
                ..Default::default()
            };
            for row in 0..2 {
                spawn_over_row(parent, row, &text_style);
            }
        });
}

fn spawn_over_row(parent: &mut ChildBuilder, row: usize, text_style: &TextStyle) {
    for column in 0..6 {
        parent
            .spawn(NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    grid_column: GridPlacement::start(column as i16 + 1),
                    width: Val::Px(30.),
                    height: Val::Px(30.),
                    margin: UiRect::all(Val::Px(4.)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn((
                    BowlTracker {
                        index: row * 6 + column,
                        style: text_style.clone(),
                        parent: parent.parent_entity(),
                    },
                    TextBundle::from_section("", text_style.clone()),
                ));
            });
    }
}

fn update_over_tracker(
    mut container_query: Query<&mut BackgroundColor>,
    mut text_node_query: Query<(&BowlTracker, &mut Text)>,
    over: Res<Over>,
) {
    for (bowl_tracker, mut text) in text_node_query.iter_mut() {
        let Ok(mut container_style) = container_query.get_mut(bowl_tracker.parent) else { continue };
        let score = over.get(bowl_tracker.index);
        if let Some(score) = score {
            *text = Text::from_section(score.value.to_string(), bowl_tracker.style.clone());
            container_style.0 = match score.scorer {
                Identity::One => Color::LIME_GREEN,
                Identity::Two => Color::CYAN,
            };
        } else {
            *text = Text::from_section("", bowl_tracker.style.clone());
            container_style.0 = Color::WHITE;
        }
    }
}

#[derive(Component)]
struct ReturnButton;

fn spawn_gameover_panel(
    mut commands: Commands,
    player_one_query: Query<&Score, With<PlayerOne>>,
    player_two_query: Query<&Score, With<PlayerTwo>>,
) {
    let player_one_score = player_one_query.single();
    let player_two_score = player_two_query.single();
    let winner = if player_one_score.0 > player_two_score.0 {
        Some(Identity::One)
    } else if player_one_score.0 < player_two_score.0 {
        Some(Identity::Two)
    } else {
        None
    };

    commands
        .spawn((
            GameoverPanel,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::rgba(0., 0., 0., 0.)),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(10.),
                        padding: UiRect::all(Val::Px(8.)),
                        border: UiRect::all(Val::Px(4.)),
                        ..Default::default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: BackgroundColor(Color::AZURE),
                    ..Default::default()
                })
                .with_children(|parent| {
                    let winner_text = if let Some(winner) = winner {
                        format!("Player {} wins!", winner)
                    } else {
                        "It's a tie!".to_string()
                    };
                    parent.spawn(TextBundle::from_section(
                        winner_text,
                        TextStyle {
                            font_size: 28.,
                            color: Color::BLACK,
                            ..Default::default()
                        },
                    ));

                    let score_text_style = TextStyle {
                        font_size: 18.,
                        color: Color::BLACK,
                        ..Default::default()
                    };
                    parent.spawn(TextBundle::from_section(
                        format!("Player One: {}", player_one_score.0),
                        score_text_style.clone(),
                    ));
                    parent.spawn(TextBundle::from_section(
                        format!("Player Two: {}", player_two_score.0),
                        score_text_style,
                    ));

                    parent
                        .spawn((
                            ReturnButton,
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(200.),
                                    height: Val::Px(65.),
                                    border: UiRect::all(Val::Px(2.)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                border_color: BorderColor(Color::BLACK),
                                background_color: BackgroundColor(Color::GRAY),
                                ..Default::default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Back to Home",
                                TextStyle {
                                    font_size: 24.0,
                                    ..Default::default()
                                },
                            ));
                        });
                });
        });
}

fn build_detect_return_selection_system<AppScreen: States + Copy>(
    return_screen: AppScreen,
) -> impl FnMut(
    Query<&Interaction, (Changed<Interaction>, With<ReturnButton>)>,
    ResMut<NextState<AppScreen>>,
) {
    move |button_query: Query<&Interaction, (Changed<Interaction>, With<ReturnButton>)>,
          mut state: ResMut<NextState<AppScreen>>| {
        for interaction in button_query.iter() {
            match interaction {
                Interaction::Pressed => {
                    state.set(return_screen);
                }
                _ => {}
            }
        }
    }
}

fn cleanup_ui(
    mut commands: Commands,
    scoreboard_query: Query<Entity, With<Scoreboard>>,
    over_scoreboard_query: Query<Entity, With<OverScoreboard>>,
    gameover_panel_query: Query<Entity, With<GameoverPanel>>,
) {
    for entity in scoreboard_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in over_scoreboard_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in gameover_panel_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, SystemSet)]
pub struct GameUISet;

pub struct GameUIPlugin<AppScreen: States, GameState: States> {
    active_screen: AppScreen,
    return_screen: AppScreen,
    gameover_state: GameState,
}

impl<AppScreen, GameState> GameUIPlugin<AppScreen, GameState>
where
    AppScreen: States,
    GameState: States,
{
    pub fn new(
        active_screen: AppScreen,
        return_screen: AppScreen,
        gameover_state: GameState,
    ) -> Self {
        GameUIPlugin {
            active_screen,
            return_screen,
            gameover_state,
        }
    }
}

impl<AppScreen, GameState> Plugin for GameUIPlugin<AppScreen, GameState>
where
    AppScreen: States + Copy,
    GameState: States + Copy,
{
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.active_screen), spawn_over_tracker)
            .add_systems(
                PostUpdate,
                (spawn_scoreboard, update_scoreboard, update_over_tracker).in_set(GameUISet),
            )
            .add_systems(OnEnter(self.gameover_state), spawn_gameover_panel)
            .add_systems(
                PostUpdate,
                build_detect_return_selection_system(self.return_screen)
                    .run_if(in_state(self.gameover_state)),
            )
            .add_systems(OnExit(self.active_screen), cleanup_ui);
    }
}
