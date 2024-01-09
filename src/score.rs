use bevy::prelude::*;

use crate::{schedule::InGameSet, Side};

// Scoreboard
const SCOREBOARD_FONT_SIZE: f32 = 42.;
const SCORE_COLOR: Color = Color::GOLD;
const SCORE_GAP: Val = Val::Px(20.0);

#[derive(Resource)]
pub struct Score {
    pub left: u32,
    pub right: u32,
}

#[derive(Component)]
struct ScoreText {
    side: Side,
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score { left: 0, right: 0 })
            .add_systems(Startup, setup_scoreboard)
            .add_systems(
                FixedUpdate,
                update_scoreboard.in_set(InGameSet::EntityUpdates),
            );
    }
}

fn setup_scoreboard(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Auto,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: SCORE_GAP,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_sections([TextSection::from_style(
                    TextStyle {
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: SCORE_COLOR,
                        ..default()
                    },
                )]))
                .insert(ScoreText { side: Side::Left });
            parent
                .spawn(TextBundle::from_sections([TextSection::from_style(
                    TextStyle {
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: SCORE_COLOR,
                        ..default()
                    },
                )]))
                .insert(ScoreText { side: Side::Right });
        });
}

fn update_scoreboard(scoreboard: Res<Score>, mut query: Query<(&mut Text, &ScoreText)>) {
    for (mut text, score_text) in &mut query {
        match score_text.side {
            Side::Left => text.sections[0].value = scoreboard.left.to_string(),
            Side::Right => text.sections[0].value = scoreboard.right.to_string(),
        }
    }
}
