mod arena;
mod camera;
mod protons;
mod utils;

use bevy::prelude::*;

use self::arena::ArenaPlugin;
use self::camera::GameCamrePlugin;
use self::protons::ProtonsPlugin;

pub struct GameMainPlugin;
impl Plugin for GameMainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(AppState::ClearScene);

        app.add_systems(OnEnter(AppState::ClearScene), clear_scene);
        app.add_systems(OnEnter(AppState::InitScene), start_gameplay);

        app.add_plugins(GameCamrePlugin);
        app.add_plugins(ArenaPlugin);
        app.add_plugins(ProtonsPlugin);
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

fn start_gameplay(mut commands: Commands) {
    commands.set_state(AppState::Game);
}
