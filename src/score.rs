use bevy::prelude::*;

use crate::{schedule::InGameSet, wall::GoalEvent, Side};

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
                (update_scoreboard, update_scores)
                    .chain()
                    .in_set(InGameSet::EntityUpdates),
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
                .spawn(TextBundle::from_sections([TextSection::new(
                    "0",
                    TextStyle {
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: SCORE_COLOR,
                        ..default()
                    },
                )]))
                .insert(ScoreText { side: Side::Left });
            parent
                .spawn(TextBundle::from_sections([TextSection::new(
                    "0",
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
    if scoreboard.is_changed() {
        for (mut text, score_text) in &mut query {
            match score_text.side {
                Side::Left => text.sections[0].value = scoreboard.left.to_string(),
                Side::Right => text.sections[0].value = scoreboard.right.to_string(),
            }
        }
    }
}

fn update_scores(mut goal_events: EventReader<GoalEvent>, mut current_scores: ResMut<Score>) {
    for event in goal_events.read() {
        // We apply the score to the opposite side to where the goal was scored
        match event.0 {
            Side::Left => current_scores.right += 1,
            Side::Right => current_scores.left += 1,
        }
    }
}

pub fn reset_scores(mut current_scores: ResMut<Score>) {
    current_scores.right = 0;
    current_scores.left = 0;
}
