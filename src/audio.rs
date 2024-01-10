use bevy::prelude::*;

use crate::{schedule::InGameSet, wall::GoalEvent, CollisionEvent};

#[derive(Resource)]
struct CollisionSound(Handle<AudioSource>);
#[derive(Resource)]
struct GoalSound(Handle<AudioSource>);

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_audio_assets).add_systems(
            FixedUpdate,
            (play_collision_sound, play_goal_sound).in_set(InGameSet::EntityUpdates),
        );
    }
}

fn load_audio_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Bounce
    let ball_collision_sound = asset_server.load("bounce.wav");
    commands.insert_resource(CollisionSound(ball_collision_sound));
    // Goal
    let ball_goal_sound = asset_server.load("goal.wav");
    commands.insert_resource(GoalSound(ball_goal_sound));
}

fn play_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    sound: Res<CollisionSound>,
) {
    // Play a sound once per frame if a collision occurred.
    if !collision_events.is_empty() {
        // This prevents events staying active on the next frame.
        collision_events.clear();
        commands.spawn(AudioBundle {
            source: sound.0.clone(),
            // auto-despawn the entity when playback finishes
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

fn play_goal_sound(
    mut commands: Commands,
    mut goal_events: EventReader<GoalEvent>,
    sound: Res<GoalSound>,
) {
    // Play a sound once per frame if a collision occurred.
    if !goal_events.is_empty() {
        // This prevents events staying active on the next frame.
        goal_events.clear();
        commands.spawn(AudioBundle {
            source: sound.0.clone(),
            // auto-despawn the entity when playback finishes
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
