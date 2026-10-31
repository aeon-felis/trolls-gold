mod arena;
mod camera;
mod utils;

use bevy::prelude::*;

use self::arena::ArenaPlugin;
use self::camera::GameCamrePlugin;

pub struct GameMainPlugin;
impl Plugin for GameMainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(AppState::ClearScene);

        app.add_systems(OnEnter(AppState::Game), clear_scene);

        app.add_plugins(GameCamrePlugin);
        app.add_plugins(ArenaPlugin);
    }
}

#[derive(States, Clone, Hash, Debug, PartialEq, Eq)]
pub enum AppState {
    ClearScene,
    InitScene,
    Game,
    GameCompleted,
}

fn clear_scene(mut commands: Commands) {
    commands.set_state(AppState::InitScene);
}
