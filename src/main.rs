use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::{
        collide_aabb::{collide, Collision},
        MaterialMesh2dBundle,
    },
};

// Play Area
const WIDTH: f32 = 600.;
const HEIGHT: f32 = 400.;
const WALL_THICKNESS: f32 = 20.;

// Colors
const PADDLE_COLOR: Color = Color::SEA_GREEN;
const BALL_COLOR: Color = Color::PURPLE;
const WALL_COLOR: Color = Color::AQUAMARINE;

// Paddles
const PADDLE_SIZE: Vec2 = Vec2::new(20., 60.);
const PADDLE_OFFSET: f32 = 40.;
const PADDLE_SPEED: f32 = 500.;

// Ball
const BALL_RADIUS: f32 = 10.;
const BALL_POSITION: Vec3 = Vec3::new(0., 0., 0.);
const BALL_SPEED: f32 = 200.;
const MAX_BOUNCE_ANGLE: f32 = 45.;

#[derive(Resource, Default)]
struct Scores {
    left: i32,
    right: i32,
}
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);
#[derive(Component)]
struct Player;
#[derive(Component)]
struct Ball;
#[derive(Component)]
struct Collider {
    bounding_box: Vec2,
}
#[derive(Component)]
struct Trigger {
    bounding_box: Vec2,
}
#[derive(Event, Default)]
struct CollisionEvent;
#[derive(Component)]
struct Goal {
    side: PaddleLocation,
    score: i32,
}
#[derive(Event, Default)]
struct GoalEvent;

fn main() {
    App::new()
        // Set up Bevy
        .add_plugins(DefaultPlugins)
        // Events
        .add_event::<CollisionEvent>()
        .add_event::<GoalEvent>()
        // Global Resources
        // Initialize both scores to 0
        .init_resource::<Scores>()
        // User Systems
        .add_systems(Startup, (setup_camera, setup_scene))
        .add_systems(
            FixedUpdate,
            (
                move_ball,
                move_player_paddle,
                limit_player_paddle,
                handle_collisions,
                handle_goals,
                // print_scores,
            ),
        )
        .run();
}

fn print_scores(goals_query: Query<&Goal>) {
    for goal in &goals_query {
        println!("{:?}: {}", goal.side, goal.score);
    }
}

fn handle_collisions(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(&Transform, &Collider)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    for (collider_transform, collider) in &collider_query {
        let collision = collide(
            ball_transform.translation,
            Vec2::new(BALL_RADIUS * 2., BALL_RADIUS * 2.),
            collider_transform.translation,
            collider.bounding_box,
        );

        if let Some(collision) = collision {
            collision_events.send_default();
            let mut reflect_y = false;

            match collision {
                Collision::Left | Collision::Right => {
                    let relative_y = (ball_transform.translation.y
                        - collider_transform.translation.y)
                        / collider.bounding_box.y;
                    let angle = relative_y * MAX_BOUNCE_ANGLE * PI / 180.;
                    let direction = if collision == Collision::Left {
                        -1.0
                    } else {
                        1.0
                    };
                    ball_velocity.x = angle.cos() * direction;
                    ball_velocity.y = angle.sin();
                }
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => { /* Do Nothing */ }
            }

            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}

fn handle_goals(
    ball_query: Query<&Transform, With<Ball>>,
    trigger_query: Query<(&Transform, &Trigger)>,
    mut goal_events: EventWriter<GoalEvent>,
) {
    let ball_transform = ball_query.single();
    for (trigger_transform, trigger) in &trigger_query {
        let collision = collide(
            ball_transform.translation,
            Vec2::new(BALL_RADIUS * 2., BALL_RADIUS * 2.),
            trigger_transform.translation,
            trigger.bounding_box,
        );
        if let Some(collision) = collision {
            goal_events.send_default()
        }
    }
}

fn move_ball(mut ball_query: Query<(&mut Transform, &Velocity), With<Ball>>, time: Res<Time>) {
    for (mut transform, velocity) in ball_query.iter_mut() {
        transform.translation += (**velocity * BALL_SPEED) * time.delta_seconds();
    }
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
    if player_transform.translation.y >= top_limit {
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
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_POSITION),
            ..default()
        },
        Ball,
        Velocity(Vec3::new(1., 0., 0.).normalize()),
    ));
    // Right Paddle (Player)
    commands
        .spawn(PaddleBundle::new(PaddleLocation::Right))
        .insert(Player);
    // Left Paddle (CPU)
    commands.spawn(PaddleBundle::new(PaddleLocation::Left));
    // Walls
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(GoalBundle::new(PaddleLocation::Left));
    commands.spawn(GoalBundle::new(PaddleLocation::Right));
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        let size = location.size();
        WallBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_translation(location.position()),
                ..default()
            },
            collider: Collider { bounding_box: size },
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
struct GoalBundle {
    transform: TransformBundle,
    goal: Goal,
    trigger: Trigger,
}

impl GoalBundle {
    fn new(location: PaddleLocation) -> GoalBundle {
        let wall_equivalent = match location {
            PaddleLocation::Left => WallLocation::Left,
            PaddleLocation::Right => WallLocation::Right,
        };
        let position = wall_equivalent.position();
        let size = wall_equivalent.size();
        GoalBundle {
            transform: TransformBundle {
                local: Transform::from_translation(Vec3::new(position.x, position.y, 0.)),
                ..default()
            },
            goal: Goal {
                side: location,
                score: 0,
            },
            trigger: Trigger { bounding_box: size },
        }
    }
}

#[derive(Bundle)]
struct PaddleBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
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
            collider: Collider {
                bounding_box: PADDLE_SIZE,
            },
        }
    }
}

#[derive(Debug)]
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
