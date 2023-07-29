use bevy::{
    prelude::{
        Added, AlignItems, App, BackgroundColor, BuildChildren, ButtonBundle, Changed,
        ChildBuilder, Color, Commands, Component, DespawnRecursiveExt, DetectChanges, Display,
        Entity, FlexDirection, GridAutoFlow, IntoSystemConfigs, JustifyContent, NextState,
        NodeBundle, OnEnter, Plugin, PositionType, PostUpdate, Query, Ref, ResMut, States, Style,
        SystemSet, Text, TextBundle, TextStyle, UiRect, Val, With, Without,
    },
    ui::{BorderColor, GridPlacement, GridTrack, Interaction},
};

use cricket_pong_base::components::{
    phase::GamePhase,
    player::{Identity, PlayerOne, PlayerTwo, Position},
    scoreboard::Scoreboard,
};

#[derive(Component)]
struct ScoreboardUI;

#[derive(Component)]
struct OverScoreboardUI;

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

fn spawn_player_scoreboard(commands: &mut Commands, position: &Position, player: Identity) {
    commands
        .spawn((
            ScoreboardUI,
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
                TextBundle::from_section("0".to_string(), score_text_style),
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
    player_one_query: Query<&Position, (Added<PlayerOne>, Without<PlayerTwo>)>,
    player_two_query: Query<&Position, (Added<PlayerTwo>, Without<PlayerOne>)>,
) {
    let Ok(position_one) = player_one_query.get_single() else { return };
    let Ok(position_two) = player_two_query.get_single() else { return };
    spawn_player_scoreboard(&mut commands, position_one, Identity::One);
    spawn_player_scoreboard(&mut commands, position_two, Identity::Two);
}

fn track_scores(
    scoreboard_query: Query<&Scoreboard, Changed<Scoreboard>>,
    mut score_count_query: Query<(&mut Text, &ScoreTracker)>,
) {
    let Ok(scoreboard) = scoreboard_query.get_single() else { return };
    let player_one_score = scoreboard.player_score(Identity::One);
    let player_two_score = scoreboard.player_score(Identity::Two);

    for (score, identity) in vec![
        (player_one_score, Identity::One),
        (player_two_score, Identity::Two),
    ] {
        for (mut text, tracker) in score_count_query.iter_mut() {
            if tracker.player == identity {
                *text = Text::from_section(score.to_string(), tracker.style.clone());
            }
        }
    }
}

fn track_positions(
    player_one_query: Query<&Position, (Changed<Position>, With<PlayerOne>, Without<PlayerTwo>)>,
    player_two_query: Query<&Position, (Changed<Position>, With<PlayerTwo>, Without<PlayerOne>)>,
    mut position_tracker_query: Query<(&mut Text, &PositionTracker), Without<ScoreTracker>>,
) {
    let position_one = player_one_query.get_single();
    let position_two = player_two_query.get_single();

    for (position, identity) in vec![(position_one, Identity::One), (position_two, Identity::Two)] {
        for (mut text, tracker) in position_tracker_query.iter_mut() {
            if tracker.player == identity {
                if let Ok(position) = position {
                    *text = Text::from_section(position.to_string(), tracker.style.clone());
                }
            }
        }
    }
}

fn spawn_over_tracker(mut commands: Commands, scoreboard_query: Query<(), Added<Scoreboard>>) {
    for _ in scoreboard_query.iter() {
        commands
            .spawn((
                OverScoreboardUI,
                NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_auto_flow: GridAutoFlow::Column,
                        grid_template_rows: vec![
                            GridTrack::min_content(),
                            GridTrack::min_content(),
                        ],
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
    scoreboard_query: Query<&Scoreboard>,
) {
    let Ok(scoreboard) = scoreboard_query.get_single() else { return };
    for (bowl_tracker, mut text) in text_node_query.iter_mut() {
        let Ok(mut container_style) = container_query.get_mut(bowl_tracker.parent) else { continue };
        let score = scoreboard.get(bowl_tracker.index);
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
    game_query: Query<Ref<GamePhase>>,
    scoreboard_query: Query<&Scoreboard>,
) {
    let Ok(game) = game_query.get_single() else { return; };
    if !game.is_changed() || *game != GamePhase::GameOver {
        return;
    }

    let Ok(scoreboard) = scoreboard_query.get_single() else { return };
    let player_one_score = scoreboard.player_score(Identity::One);
    let player_two_score = scoreboard.player_score(Identity::Two);
    let winner = if player_one_score > player_two_score {
        Some(Identity::One)
    } else if player_one_score < player_two_score {
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
                        format!("Player One: {}", player_one_score),
                        score_text_style.clone(),
                    ));
                    parent.spawn(TextBundle::from_section(
                        format!("Player Two: {}", player_two_score),
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
    scoreboard_query: Query<Entity, With<ScoreboardUI>>,
    over_scoreboard_query: Query<Entity, With<OverScoreboardUI>>,
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

pub struct GameUIPlugin<AppScreen: States> {
    return_screen: AppScreen,
}

impl<AppScreen> GameUIPlugin<AppScreen>
where
    AppScreen: States,
{
    pub fn new(return_screen: AppScreen) -> Self {
        GameUIPlugin { return_screen }
    }
}

impl<AppScreen> Plugin for GameUIPlugin<AppScreen>
where
    AppScreen: States + Copy,
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                spawn_scoreboard,
                track_scores,
                track_positions,
                spawn_over_tracker,
                update_over_tracker,
            )
                .in_set(GameUISet),
        )
        .add_systems(PostUpdate, spawn_gameover_panel)
        .add_systems(
            PostUpdate,
            build_detect_return_selection_system(self.return_screen),
        )
        .add_systems(OnEnter(self.return_screen), cleanup_ui);
    }
}