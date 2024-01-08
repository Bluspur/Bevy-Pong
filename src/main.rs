mod audio;
mod ball;
mod paddle;
mod schedule;
pub mod wall;

use audio::AudioPlugin;
use ball::BallPlugin;
use bevy::prelude::*;
use paddle::PaddlePlugin;
use schedule::{InGameSet, SchedulePlugin};
use wall::WallPlugin;

// Play Area
const WIDTH: f32 = 600.;
const HEIGHT: f32 = 400.;

// Scoreboard
const SCOREBOARD_FONT_SIZE: f32 = 42.;
const TEXT_COLOR: Color = Color::RED;
const SCORE_COLOR: Color = Color::BEIGE;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

#[derive(Resource)]
struct Score {
    left: u32,
    right: u32,
}

#[derive(Component)]
struct ScoreText {
    side: Side,
}

#[derive(Default, Copy, Clone)]
pub enum Side {
    #[default]
    Left,
    Right,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);

#[derive(Event, Default)]
struct CollisionEvent;
#[derive(Debug, Component)]
struct Collider {
    bounding_box: Vec2,
}

fn main() {
    App::new()
        // Set up Bevy
        .add_plugins(DefaultPlugins)
        .insert_resource(Score { left: 0, right: 0 })
        // Events
        .add_event::<CollisionEvent>()
        // User Systems
        .add_plugins((
            BallPlugin,
            WallPlugin,
            PaddlePlugin,
            AudioPlugin,
            SchedulePlugin,
        ))
        .add_systems(Startup, (setup_camera, setup_scoreboard))
        .add_systems(
            FixedUpdate,
            update_scoreboard.in_set(InGameSet::EntityUpdates),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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
                column_gap: Val::Px(20.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_sections([TextSection::from_style(
                    TextStyle {
                        font_size: 72.0,
                        color: Color::GOLD,
                        ..default()
                    },
                )]))
                .insert(ScoreText { side: Side::Left });
            parent
                .spawn(TextBundle::from_sections([TextSection::from_style(
                    TextStyle {
                        font_size: 72.0,
                        color: Color::GOLD,
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
