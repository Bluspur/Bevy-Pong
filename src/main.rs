#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod audio;
mod ball;
mod menu;
mod paddle;
mod reset;
mod schedule;
mod score;
pub mod wall;

use bevy::prelude::*;

// Play Area
const WIDTH: f32 = 600.;
const HEIGHT: f32 = 400.;

const TIME_TO_SERVE: f32 = 1.;

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

#[derive(Resource)]
struct ServeTimer {
    timer: Timer,
}
#[derive(Resource, Deref, DerefMut)]
struct IsFirstRun(bool);

impl ServeTimer {
    fn new() -> Self {
        Self {
            timer: Timer::from_seconds(TIME_TO_SERVE, TimerMode::Once),
        }
    }
}

impl Default for ServeTimer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default, Resource)]
enum ServeDirection {
    Left,
    #[default]
    Right,
}

impl ServeDirection {
    fn opposite(&self) -> Self {
        match self {
            ServeDirection::Left => ServeDirection::Right,
            ServeDirection::Right => ServeDirection::Left,
        }
    }
}

fn main() {
    App::new()
        // Set up Bevy
        .add_plugins(DefaultPlugins)
        .init_resource::<ServeDirection>()
        // Resources
        .init_resource::<ServeTimer>()
        .insert_resource(IsFirstRun(true))
        // Events
        .add_event::<CollisionEvent>()
        // User Systems
        .add_plugins((
            ball::BallPlugin,
            wall::WallPlugin,
            paddle::PaddlePlugin,
            audio::AudioPlugin,
            schedule::SchedulePlugin,
            score::ScorePlugin,
            menu::MenuPlugin,
            reset::ResetBundle,
        ))
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(schedule::GameState::Playing), update_first_play)
        .add_systems(
            FixedUpdate,
            open_menu_input
                .in_set(schedule::InGameSet::Input)
                .run_if(in_state(schedule::GameState::Playing)),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn open_menu_input(
    input: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<schedule::GameState>>,
) {
    if input.pressed(KeyCode::Escape) {
        game_state.set(schedule::GameState::Menu)
    }
}

fn update_first_play(mut is_first_run: ResMut<IsFirstRun>) {
    **is_first_run = false;
}
