use bevy::prelude::*;

use crate::{schedule::InGameSet, score::Score, Collider, Side, HEIGHT, WIDTH};

const GOAL_COLOR: Color = Color::DARK_GRAY;
const COLOR: Color = Color::AQUAMARINE;
const THICKNESS: f32 = 20.;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GoalEvent>()
            .add_systems(Startup, spawn_walls)
            .add_systems(FixedUpdate, update_scores.in_set(InGameSet::EntityUpdates));
    }
}

fn spawn_walls(mut commands: Commands) {
    // Walls
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(GoalBundle::new(WallLocation::Right, Side::Right));
    commands.spawn(GoalBundle::new(WallLocation::Left, Side::Left));
}

#[derive(Component)]
pub struct Goal {
    pub side: Side,
}

#[derive(Event, Default)]
pub struct GoalEvent(pub Side);

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
                    color: COLOR,
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

#[derive(Bundle)]
struct GoalBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    goal: Goal,
}

impl GoalBundle {
    fn new(location: WallLocation, side: Side) -> GoalBundle {
        let size = location.size();
        GoalBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: GOAL_COLOR,
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_translation(location.position()),
                ..default()
            },
            collider: Collider { bounding_box: size },
            goal: Goal { side },
        }
    }
}

fn update_scores(mut goal_events: EventReader<GoalEvent>, mut current_scores: ResMut<Score>) {
    for event in goal_events.read() {
        match event.0 {
            Side::Left => current_scores.left += 1,
            Side::Right => current_scores.right += 1,
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
        let offset = THICKNESS / 2.0;
        match self {
            WallLocation::Top => Vec3::new(0., (HEIGHT / 2.0) + offset, 0.),
            WallLocation::Bottom => Vec3::new(0., -(HEIGHT / 2.0) - offset, 0.),
            // Unused but good for testing
            WallLocation::Left => Vec3::new(-(WIDTH / 2.0) - offset, 0., 0.),
            WallLocation::Right => Vec3::new((WIDTH / 2.0) + offset, 0., 0.),
        }
    }
    fn size(&self) -> Vec2 {
        match self {
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(WIDTH + THICKNESS * 2.0, THICKNESS)
            }
            WallLocation::Left | WallLocation::Right => Vec2::new(THICKNESS, HEIGHT),
        }
    }
}
