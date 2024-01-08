use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    Input,
    EntityUpdates,
    CollisionDetection,
    DespawnEntities,
    UIAudioProcessing,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            FixedUpdate,
            (
                InGameSet::DespawnEntities,
                // Flush Point
                InGameSet::Input,
                InGameSet::EntityUpdates,
                InGameSet::CollisionDetection,
            )
                .chain(),
        )
        .add_systems(
            FixedUpdate,
            apply_deferred
                .after(InGameSet::DespawnEntities)
                .before(InGameSet::Input),
        );
    }
}
