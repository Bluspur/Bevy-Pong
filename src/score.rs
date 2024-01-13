use bevy::prelude::*;

use crate::{schedule::InGameSet, wall::GoalEvent, Side, HEIGHT};

// Scoreboard
const SCOREBOARD_FONT_SIZE: f32 = 72.;
const SCORE_COLOR: Color = Color::GRAY;
const SCORE_GAP: f32 = 60.;

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
            .add_systems(Startup, setup_scoreboard_worldspace)
            .add_systems(
                FixedUpdate,
                (update_scoreboard, update_scores)
                    .chain()
                    .in_set(InGameSet::EntityUpdates),
            );
    }
}

// Equivalent to the old way, but in world space so scores can be behind the ball
fn setup_scoreboard_worldspace(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/PixelifySans-VariableFont_wght.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: SCOREBOARD_FONT_SIZE,
        color: SCORE_COLOR,
    };
    commands.spawn((
        Text2dBundle {
            transform: Transform::from_translation(Vec3 {
                x: -SCORE_GAP / 2.,
                y: HEIGHT / 2.,
                z: -1.,
            }),
            text: Text::from_section("0", text_style.clone()),
            text_anchor: bevy::sprite::Anchor::TopRight,
            ..default()
        },
        ScoreText { side: Side::Left },
    ));
    commands.spawn((
        Text2dBundle {
            transform: Transform::from_translation(Vec3 {
                x: SCORE_GAP / 2.,
                y: HEIGHT / 2.,
                z: -1.,
            }),
            text: Text::from_section("0", text_style.clone()),
            text_anchor: bevy::sprite::Anchor::TopLeft,
            ..default()
        },
        ScoreText { side: Side::Right },
    ));
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
