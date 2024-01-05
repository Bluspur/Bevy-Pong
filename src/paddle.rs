use bevy::prelude::*;

use crate::{Collider, Side, HEIGHT, WIDTH};

const COLOR: Color = Color::SEA_GREEN;
const SIZE: Vec2 = Vec2::new(20., 60.);
const OFFSET: f32 = 40.;
const SPEED: f32 = 500.;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_paddles)
            .add_systems(FixedUpdate, (move_player_paddle, limit_player_paddle));
    }
}

#[derive(Component)]
struct Player;

fn spawn_paddles(mut commands: Commands) {
    // Right Paddle (Player)
    commands
        .spawn(PaddleBundle::new(Side::Right))
        .insert(Player);
    // Left Paddle (CPU)
    commands.spawn(PaddleBundle::new(Side::Left));
}

fn move_player_paddle(
    mut player_query: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_transform = player_query
        .get_single_mut()
        .expect("There should be a single player");
    let mut translation_delta = Vec3::splat(0.);
    if input.pressed(KeyCode::Up) {
        translation_delta.y += SPEED * time.delta_seconds();
    }
    if input.pressed(KeyCode::Down) {
        translation_delta.y -= SPEED * time.delta_seconds();
    }
    player_transform.translation += translation_delta;
}

fn limit_player_paddle(mut player_query: Query<&mut Transform, With<Player>>) {
    let mut player_transform = player_query
        .get_single_mut()
        .expect("There should be a single player");
    let top_limit = HEIGHT / 2.0 - SIZE.y / 2.0;
    let bottom_limit = -(HEIGHT / 2.0) + SIZE.y / 2.0;
    // Top Limit
    if player_transform.translation.y >= top_limit {
        player_transform.translation.y = top_limit;
    } else if player_transform.translation.y < bottom_limit {
        player_transform.translation.y = bottom_limit;
    }
}

#[derive(Bundle)]
struct PaddleBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
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
