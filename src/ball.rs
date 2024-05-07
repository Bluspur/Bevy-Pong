use std::f32::consts::PI;

use bevy::{
    math::{
        bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
        primitives::Circle,
    },
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};

use crate::{
    schedule::InGameSet,
    wall::{Goal, GoalEvent},
    ServeDirection, ServeTimer,
};
use crate::{Collider, CollisionEvent, Velocity};

const COLOR: Color = Color::WHITE;
const RADIUS: f32 = 10.;
const SPEED: f32 = 400.;
const MAX_BOUNCE_ANGLE: f32 = 70.;
const START_POSITION: Vec3 = Vec3::new(0., 0., 0.);

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(
                FixedUpdate,
                (serve_ball, move_ball)
                    .chain()
                    .in_set(InGameSet::EntityUpdates),
            )
            .add_systems(
                FixedUpdate,
                handle_collisions.in_set(InGameSet::CollisionDetection),
            )
            .add_systems(
                FixedUpdate,
                reset_ball_goal.in_set(InGameSet::ResetEntities),
            );
    }
}

#[derive(Debug, Component)]
pub struct Ball;

#[derive(Bundle)]
struct BallBundle<M: Material2d> {
    ball: Ball,
    mesh: MaterialMesh2dBundle<M>,
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(RADIUS)).into(),
            material: materials.add(ColorMaterial::from(COLOR)),
            transform: Transform::from_translation(START_POSITION),
            ..default()
        },
        Ball,
        Velocity(Vec3::ZERO),
    ));
}

fn serve_ball(
    mut ball_query: Query<&mut Velocity, With<Ball>>,
    time: Res<Time>,
    mut serve_timer: ResMut<ServeTimer>,
    mut serve_direction: ResMut<ServeDirection>,
) {
    if serve_timer.timer.tick(time.delta()).just_finished() {
        let direction = match *serve_direction {
            ServeDirection::Left => Vec3::NEG_X,
            ServeDirection::Right => Vec3::X,
        };
        for mut ball_velocity in &mut ball_query {
            ball_velocity.0 = direction;
        }
        *serve_direction = serve_direction.opposite();
    }
}

fn move_ball(mut ball_query: Query<(&mut Transform, &Velocity), With<Ball>>, time: Res<Time>) {
    for (mut transform, velocity) in ball_query.iter_mut() {
        transform.translation += (**velocity * SPEED) * time.delta_seconds();
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest = wall.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

fn handle_collisions(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(&Transform, &Collider, Option<&Goal>)>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut goal_events: EventWriter<GoalEvent>,
) {
    if let Ok((mut ball_velocity, ball_transform)) = ball_query.get_single_mut() {
        for (collider_transform, collider, maybe_goal) in &collider_query {
            let collision = collide_with_side(
                BoundingCircle::new(ball_transform.translation.truncate(), RADIUS),
                Aabb2d::new(
                    collider_transform.translation.truncate(),
                    collider.bounding_box / 2.,
                ),
            );

            if let Some(collision) = collision {
                // Handle goals
                if let Some(goal) = maybe_goal {
                    goal_events.send(GoalEvent(goal.side));
                    return;
                }

                // Handle collisions with walls or paddle
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
                }

                if reflect_y {
                    ball_velocity.y = -ball_velocity.y;
                }
            }
        }
    }
}

pub fn reset_ball_goal(
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    mut serve_timer: ResMut<ServeTimer>,
    mut goal_event: EventReader<GoalEvent>,
) {
    for _ in goal_event.read() {
        // Reset the ball
        for (mut ball_transform, mut ball_velocity) in &mut ball_query {
            ball_transform.translation = Vec3::ZERO;
            ball_velocity.0 = Vec3::ZERO;
        }

        serve_timer.timer.reset()
    }
}

pub fn reset_ball(
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    mut serve_timer: ResMut<ServeTimer>,
    mut serve_direction: ResMut<ServeDirection>,
) {
    for (mut ball_transform, mut ball_velocity) in &mut ball_query {
        ball_transform.translation = Vec3::ZERO;
        ball_velocity.0 = Vec3::ZERO;
    }

    serve_timer.timer.reset();
    *serve_direction = ServeDirection::Right;
}
