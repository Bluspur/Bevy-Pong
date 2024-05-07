use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Menu,
    Reset,
    Playing,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    Input,
    EntityUpdates,
    CollisionDetection,
    ResetEntities,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .configure_sets(
                FixedUpdate,
                (
                    InGameSet::ResetEntities,
                    // Flush Point
                    InGameSet::Input,
                    InGameSet::EntityUpdates,
                    InGameSet::CollisionDetection,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                FixedUpdate,
                apply_deferred
                    .after(InGameSet::ResetEntities)
                    .before(InGameSet::Input),
            );
    }
}
