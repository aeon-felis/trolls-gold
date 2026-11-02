mod arena;
mod camera;
mod player;
mod player_control;
mod protons;
mod utils;

use bevy::prelude::*;
use bevy_rapier3d::geometry::Group;

use self::arena::ArenaPlugin;
use self::camera::GameCamrePlugin;
use self::player::PlayerPlugin;
use self::player_control::PlayerControlPlugin;
use self::protons::ProtonsPlugin;

pub struct GameMainPlugin;
impl Plugin for GameMainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(AppState::ClearScene);

        app.add_systems(OnEnter(AppState::ClearScene), clear_scene);
        app.add_systems(OnEnter(AppState::InitScene), start_gameplay);

        app.add_plugins(GameCamrePlugin);
        app.add_plugins(ArenaPlugin);
        app.add_plugins(PlayerPlugin);
        app.add_plugins(PlayerControlPlugin);
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

#[derive(Component, Default, Clone)]
pub struct ClearableObject;

pub const PLAYER_COLLISION_GROUP: Group = Group::GROUP_1;
pub const ARENA_COLLISION_GROUP: Group = Group::GROUP_2;
pub const PARTICLE_COLLISION_GROUP: Group = Group::GROUP_3;

fn clear_scene(mut commands: Commands) {
    commands.set_state(AppState::InitScene);
}

fn start_gameplay(mut commands: Commands) {
    commands.set_state(AppState::Game);
}
