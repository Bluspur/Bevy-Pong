use bevy::prelude::*;

use crate::ball::reset_ball;
use crate::paddle::reset_paddles;
use crate::schedule::GameState;
use crate::score::reset_scores;

pub struct ResetBundle;

impl Plugin for ResetBundle {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Reset),
            (reset_ball, reset_paddles, reset_scores, transition).chain(),
        );
    }
}

fn transition(mut gamestate: ResMut<NextState<GameState>>) {
    gamestate.set(GameState::Playing);
}
