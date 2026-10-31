use bevy::prelude::*;

use crate::AppState;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InitScene), setup_arena);
    }
}

fn setup_arena() {}
