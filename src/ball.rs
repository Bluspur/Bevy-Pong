use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::{
        collide_aabb::{collide, Collision},
        Material2d, MaterialMesh2dBundle,
    },
};

use crate::wall::{Goal, GoalEvent};
use crate::{Collider, CollisionEvent, Velocity};

const COLOR: Color = Color::PURPLE;
const RADIUS: f32 = 10.;
const SPEED: f32 = 400.;
const MAX_BOUNCE_ANGLE: f32 = 70.;
const START_POSITION: Vec3 = Vec3::new(0., 0., 0.);

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball).add_systems(
            FixedUpdate,
            (handle_collisions, handle_goals, move_ball, despawn_ball),
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
            mesh: meshes.add(shape::Circle::new(RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(COLOR)),
            transform: Transform::from_translation(START_POSITION),
            ..default()
        },
        Ball,
        Velocity(Vec3::X),
    ));
}

fn move_ball(mut ball_query: Query<(&mut Transform, &Velocity), With<Ball>>, time: Res<Time>) {
    for (mut transform, velocity) in ball_query.iter_mut() {
        transform.translation += (**velocity * SPEED) * time.delta_seconds();
    }
}

fn handle_collisions(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(&Transform, &Collider), Without<Goal>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    if let Ok((mut ball_velocity, ball_transform)) = ball_query.get_single_mut() {
        for (collider_transform, collider) in &collider_query {
            let collision = collide(
                ball_transform.translation,
                Vec2::new(RADIUS * 2., RADIUS * 2.),
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
}

fn handle_goals(
    ball_query: Query<&Transform, With<Ball>>,
    trigger_query: Query<(&Transform, &Collider, &Goal)>,
    mut goal_events: EventWriter<GoalEvent>,
) {
    if let Ok(ball_transform) = ball_query.get_single() {
        for (trigger_transform, trigger, goal) in &trigger_query {
            let collision = collide(
                ball_transform.translation,
                Vec2::new(RADIUS * 2., RADIUS * 2.),
                trigger_transform.translation,
                trigger.bounding_box,
            );
            if collision.is_some() {
                goal_events.send(GoalEvent(goal.side))
            }
        }
    }
}

fn despawn_ball(
    mut commands: Commands,
    ball_query: Query<Entity, With<Ball>>,
    mut goal_event: EventReader<GoalEvent>,
) {
    for _ in goal_event.read() {
        for entity in &ball_query {
            commands.entity(entity).despawn_recursive();
        }
    }
}
