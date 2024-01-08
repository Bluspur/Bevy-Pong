use bevy::prelude::*;

use crate::ball::Ball;
use crate::{Collider, Side, Velocity, HEIGHT, WIDTH};

const COLOR: Color = Color::SEA_GREEN;
const SIZE: Vec2 = Vec2::new(20., 60.);
const OFFSET: f32 = 40.;
const SPEED: f32 = 500.;

const CPU_DIFFERENCE_TOLERANCE: f32 = 7.;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_paddles).add_systems(
            FixedUpdate,
            (handle_player_input, move_paddles, cpu_matches_ball),
        );
    }
}

#[derive(Component)]
struct Paddle;
#[derive(Component)]
struct Player;
#[derive(Component)]
struct CPU;

fn spawn_paddles(mut commands: Commands) {
    // Right Paddle (Player)
    commands.spawn((PaddleBundle::new(Side::Right), Player));
    // Left Paddle (CPU)
    commands.spawn((PaddleBundle::new(Side::Left), CPU));
}

fn handle_player_input(
    mut player_paddle_query: Query<&mut Velocity, With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    let mut player_velocity = player_paddle_query
        .get_single_mut()
        .expect("There should only be a single player");

    let mut vertical_direction = 0.;
    if input.pressed(KeyCode::Up) {
        vertical_direction += 1.;
    }
    if input.pressed(KeyCode::Down) {
        vertical_direction -= 1.;
    }
    player_velocity.y = vertical_direction;
}

fn cpu_matches_ball(
    mut cpu_paddle_query: Query<(&Transform, &mut Velocity), With<CPU>>,
    ball_query: Query<&Transform, With<Ball>>,
) {
    let (cpu_transform, mut cpu_velocity) = cpu_paddle_query
        .get_single_mut()
        .expect("There should only be a single CPU");
    if let Ok(ball_transform) = ball_query.get_single() {
        let paddle_ball_height_difference =
            cpu_transform.translation.y - ball_transform.translation.y;
        cpu_velocity.y = match paddle_ball_height_difference {
            diff if diff > CPU_DIFFERENCE_TOLERANCE => -diff / 100.,
            diff if diff < -CPU_DIFFERENCE_TOLERANCE => -diff / 100.,
            _ => 0.,
        }
    } else {
        // Fixes the paddle shooting off after the ball dissapears
        cpu_velocity.y = 0.;
    }
}

// Updates the position of the paddle with respect to the bottom and top of the play area
fn move_paddles(
    mut paddle_query: Query<(&mut Transform, &Velocity), With<Paddle>>,
    time: Res<Time>,
) {
    let top_bound = HEIGHT / 2.0 - SIZE.y / 2.0;
    let bottom_bound = -(HEIGHT / 2.0) + SIZE.y / 2.0;
    for (mut paddle_transform, paddle_velocity) in &mut paddle_query {
        let new_position =
            paddle_transform.translation.y + (paddle_velocity.y * SPEED * time.delta_seconds());
        paddle_transform.translation.y = new_position.clamp(bottom_bound, top_bound);
    }
}

#[derive(Bundle)]
struct PaddleBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    velocity: Velocity,
    paddle: Paddle,
}

impl PaddleBundle {
    fn new(side: Side) -> PaddleBundle {
        PaddleBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: COLOR,
                    custom_size: Some(SIZE),
                    ..default()
                },
                transform: Transform::from_translation(position(side)),
                ..default()
            },
            collider: Collider { bounding_box: SIZE },
            velocity: Velocity(Vec3::ZERO),
            paddle: Paddle,
        }
    }
}

fn position(side: Side) -> Vec3 {
    let half_width = SIZE.x / 2.0;
    match side {
        Side::Left => Vec3::new(-(WIDTH / 2.0) + (OFFSET + half_width), 0., 0.),
        Side::Right => Vec3::new((WIDTH / 2.0) - (OFFSET + half_width), 0., 0.),
    }
}
