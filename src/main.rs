mod audio;
mod ball;
mod paddle;
mod schedule;
mod score;
pub mod wall;

use audio::AudioPlugin;
use ball::BallPlugin;
use bevy::prelude::*;
use paddle::PaddlePlugin;
use schedule::SchedulePlugin;
use score::ScorePlugin;
use wall::WallPlugin;

// Play Area
const WIDTH: f32 = 600.;
const HEIGHT: f32 = 400.;

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
        // Events
        .add_event::<CollisionEvent>()
        // User Systems
        .add_plugins((
            BallPlugin,
            WallPlugin,
            PaddlePlugin,
            AudioPlugin,
            SchedulePlugin,
            ScorePlugin,
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
