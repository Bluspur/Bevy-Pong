mod audio;
mod ball;
mod paddle;
pub mod wall;

use audio::AudioPlugin;
use ball::BallPlugin;
use bevy::prelude::*;
use paddle::PaddlePlugin;
use wall::WallPlugin;

// Play Area
const WIDTH: f32 = 600.;
const HEIGHT: f32 = 400.;

#[derive(Resource)]
struct Score {
    left: u32,
    right: u32,
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
        .add_plugins((BallPlugin, WallPlugin, PaddlePlugin, AudioPlugin))
        .add_systems(Startup, (setup_camera, setup_scoreboard))
        .add_systems(FixedUpdate, update_scoreboard)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_scoreboard(mut commands: Commands) {
    // Scoreboard
    const SCOREBOARD_FONT_SIZE: f32 = 42.;
    const TEXT_COLOR: Color = Color::RED;
    const SCORE_COLOR: Color = Color::BEIGE;
    const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
    );
}

fn update_scoreboard(scoreboard: Res<Score>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.left.to_string();
}
