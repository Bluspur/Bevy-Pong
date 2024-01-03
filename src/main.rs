use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

// Play Area
const WIDTH: f32 = 600.;
const HEIGHT: f32 = 400.;
const WALL_THICKNESS: f32 = 20.;

// Colors
const PADDLE_COLOR: Color = Color::SEA_GREEN;
const BALL_COLOR: Color = Color::PURPLE;
const WALL_COLOR: Color = Color::AQUAMARINE;

// Paddles
const PADDLE_SIZE: Vec2 = Vec2::new(50., 100.);
const PADDLE_OFFSET: f32 = 20.;
const PADDLE_SPEED: f32 = 250.;

// Ball
const BALL_RADIUS: f32 = 10.;
const BALL_POSITION: Vec3 = Vec3::new(0., 0., 0.);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);
#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_scene))
        .add_systems(Update, (move_player_paddle, limit_player_paddle))
        .run();
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
        translation_delta.y += PADDLE_SPEED * time.delta_seconds();
    }
    if input.pressed(KeyCode::Down) {
        translation_delta.y -= PADDLE_SPEED * time.delta_seconds();
    }
    player_transform.translation += translation_delta;
}

fn limit_player_paddle(mut player_query: Query<&mut Transform, With<Player>>) {
    let mut player_transform = player_query
        .get_single_mut()
        .expect("There should be a single player");
    let top_limit = HEIGHT / 2.0 - PADDLE_SIZE.y / 2.0;
    let bottom_limit = -(HEIGHT / 2.0) + PADDLE_SIZE.y / 2.0;
    // Top Limit
    if player_transform.translation.y > top_limit {
        player_transform.translation.y = top_limit;
    } else if player_transform.translation.y < bottom_limit {
        player_transform.translation.y = bottom_limit;
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Ball
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
        material: materials.add(ColorMaterial::from(BALL_COLOR)),
        transform: Transform::from_translation(BALL_POSITION),
        ..default()
    });
    // Right Paddle (Player)
    commands
        .spawn(PaddleBundle::new(PaddleLocation::Right))
        .insert(Player);
    // Left Paddle (CPU)
    commands.spawn(PaddleBundle::new(PaddleLocation::Left));
    // Walls
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(location.size()),
                    ..default()
                },
                transform: Transform::from_translation(location.position()),
                ..default()
            },
        }
    }
}

enum WallLocation {
    Top,
    Bottom,
    Left,
    Right,
}

impl WallLocation {
    fn position(&self) -> Vec3 {
        let offset = WALL_THICKNESS / 2.0;
        match self {
            WallLocation::Top => Vec3::new(0., (HEIGHT / 2.0) + offset, 0.),
            WallLocation::Bottom => Vec3::new(0., -(HEIGHT / 2.0) - offset, 0.),
            WallLocation::Left => Vec3::new(-(WIDTH / 2.0) - offset, 0., 0.),
            WallLocation::Right => Vec3::new((WIDTH / 2.0) + offset, 0., 0.),
        }
    }
    fn size(&self) -> Vec2 {
        match self {
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(WIDTH + WALL_THICKNESS * 2.0, WALL_THICKNESS)
            }
            WallLocation::Left | WallLocation::Right => Vec2::new(WALL_THICKNESS, HEIGHT),
        }
    }
}

#[derive(Bundle)]
struct PaddleBundle {
    sprite_bundle: SpriteBundle,
}

impl PaddleBundle {
    fn new(location: PaddleLocation) -> PaddleBundle {
        PaddleBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: PADDLE_COLOR,
                    custom_size: Some(PADDLE_SIZE),
                    ..default()
                },
                transform: Transform::from_translation(location.position()),
                ..default()
            },
        }
    }
}

enum PaddleLocation {
    Left,
    Right,
}

impl PaddleLocation {
    fn position(&self) -> Vec3 {
        let half_width = PADDLE_SIZE.x / 2.0;
        match self {
            PaddleLocation::Left => {
                Vec3::new(-(WIDTH / 2.0) + (PADDLE_OFFSET + half_width), 0., 0.)
            }
            PaddleLocation::Right => {
                Vec3::new((WIDTH / 2.0) - (PADDLE_OFFSET + half_width), 0., 0.)
            }
        }
    }
}
