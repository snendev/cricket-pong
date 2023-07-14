use bevy_ecs::{
    prelude::Component,
    system::{Commands, Query},
};
use bevy_hierarchy::BuildChildren;
use bevy_render::prelude::Color;
use bevy_text::{prelude::TextStyle, Text};
use bevy_ui::{
    prelude::{
        AlignItems, BackgroundColor, FlexDirection, NodeBundle, PositionType, Style, TextBundle,
        Val,
    },
    UiRect,
};

use cricket_pong_base::{Identity, Player};

#[derive(Component)]
pub(crate) struct ScoreCount {
    pub value: u16,
    pub player: Identity,
    pub style: TextStyle,
}

fn spawn_player_scoreboard(commands: &mut Commands, player: &Player) {
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
            let score_style = TextStyle {
                font_size: 28.,
                color: Color::BLACK,
                ..Default::default()
            };
            parent.spawn((
                ScoreCount {
                    value: player.score,
                    player: player.id,
                    style: score_style.clone(),
                },
                TextBundle::from_section(player.score.to_string(), score_style),
            ));
            parent.spawn(TextBundle::from_section(
                player.objective.to_string(),
                TextStyle {
                    color: Color::BLACK,
                    ..Default::default()
                },
            ));
        });
}

pub(crate) fn spawn_scoreboard(mut commands: Commands, players_query: Query<&Player>) {
    let Some(player_one) = players_query
        .iter()
        .find(|player| player.id == Identity::One) else { return };
    let Some(player_two) = players_query
        .iter()
        .find(|player| player.id == Identity::Two) else { return };
    spawn_player_scoreboard(&mut commands, player_one);
    spawn_player_scoreboard(&mut commands, player_two);
}

pub(crate) fn update_scoreboard(
    players_query: Query<&Player>,
    mut score_count_query: Query<(&mut Text, &ScoreCount)>,
) {
    for player in players_query.iter() {
        for (mut text, score) in score_count_query.iter_mut() {
            if score.player == player.id {
                *text = Text::from_section(player.score.to_string(), score.style.clone());
            }
        }
    }
}
