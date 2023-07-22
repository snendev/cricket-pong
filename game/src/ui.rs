use bevy_ecs::{
    prelude::{Commands, Component, Entity, Query, Res},
    query::Without,
};
use bevy_hierarchy::BuildChildren;
use bevy_render::prelude::Color;
use bevy_text::{prelude::TextStyle, Text};
use bevy_ui::{
    prelude::{
        AlignItems, BackgroundColor, FlexDirection, NodeBundle, PositionType, Style, TextBundle,
        Val,
    },
    Display, GridAutoFlow, JustifyContent, UiRect,
};

use cricket_pong_base::{Identity, Player, Position};

use crate::scoring::Over;

#[derive(Component)]
pub(crate) struct ScoreTracker {
    pub player: Identity,
    pub style: TextStyle,
}

#[derive(Component)]
pub(crate) struct PositionTracker {
    pub player: Identity,
    pub style: TextStyle,
}

fn spawn_player_scoreboard(commands: &mut Commands, player: &Player, position: &Position) {
    commands
        .spawn(NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(12.)),
                column_gap: Val::Px(4.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                left: match player.id {
                    Identity::One => Val::Percent(0.),
                    Identity::Two => Val::Auto,
                },
                right: match player.id {
                    Identity::One => Val::Auto,
                    Identity::Two => Val::Percent(0.),
                },
                ..Default::default()
            },
            background_color: BackgroundColor(Color::AZURE),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("Player {}", player.id),
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
                    player: player.id,
                    style: score_text_style.clone(),
                },
                TextBundle::from_section(player.score.to_string(), score_text_style),
            ));
            parent.spawn((
                PositionTracker {
                    player: player.id,
                    style: position_text_style.clone(),
                },
                TextBundle::from_section(position.to_string(), position_text_style),
            ));
        });
}

pub(crate) fn spawn_scoreboard(mut commands: Commands, players_query: Query<(&Player, &Position)>) {
    let Some((player_one, position_one)) = players_query
        .iter()
        .find(|(player, _): &(&Player, &Position)| player.id == Identity::One) else { return };
    let Some((player_two, position_two)) = players_query
        .iter()
        .find(|(player, _)| player.id == Identity::Two) else { return };
    spawn_player_scoreboard(&mut commands, player_one, position_one);
    spawn_player_scoreboard(&mut commands, player_two, position_two);
}

pub(crate) fn update_scoreboard(
    players_query: Query<(&Player, &Position)>,
    mut score_count_query: Query<(&mut Text, &ScoreTracker)>,
    mut position_tracker_query: Query<(&mut Text, &PositionTracker), Without<ScoreTracker>>,
) {
    for (player, position) in players_query.iter() {
        for (mut text, tracker) in score_count_query.iter_mut() {
            if tracker.player == player.id {
                *text = Text::from_section(player.score.to_string(), tracker.style.clone());
            }
        }
        for (mut text, tracker) in position_tracker_query.iter_mut() {
            if tracker.player == player.id {
                *text = Text::from_section(position.to_string(), tracker.style.clone());
            }
        }
    }
}

#[derive(Component)]
pub(crate) struct BowlTracker {
    pub index: usize,
    pub style: TextStyle,
    pub parent: Entity,
}

pub(crate) fn spawn_over_tracker(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_auto_flow: GridAutoFlow::Column,
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.),
                left: Val::Px(0.),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NAVY),
            ..Default::default()
        })
        .with_children(|parent| {
            let text_style = TextStyle {
                font_size: 28.,
                color: Color::BLACK,
                ..Default::default()
            };
            for index in 0..6 {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
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
                                index,
                                style: text_style.clone(),
                                parent: parent.parent_entity(),
                            },
                            TextBundle::from_section("", text_style.clone()),
                        ));
                    });
            }
        });
}

pub(crate) fn update_over_tracker(
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
                Position::Fielder => Color::LIME_GREEN,
                Position::Batter => Color::CYAN,
            };
        } else {
            *text = Text::from_section("", bowl_tracker.style.clone());
            container_style.0 = Color::WHITE;
        }
    }
}
