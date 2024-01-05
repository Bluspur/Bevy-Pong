mod ball;
mod paddle;
pub mod wall;

use ball::BallPlugin;
use bevy::prelude::*;
use paddle::PaddlePlugin;
use wall::{GoalEvent, WallPlugin};

// Play Area
const WIDTH: f32 = 600.;
const HEIGHT: f32 = 400.;

// Useful for Scores
#[derive(Default, Copy, Clone)]
pub enum Side {
    #[default]
    Left,
    Right,
}

// Wrapper Component
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);

// Collisions
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
        .add_event::<GoalEvent>()
        // User Systems
        .add_plugins((BallPlugin, WallPlugin, PaddlePlugin))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
