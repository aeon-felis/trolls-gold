mod arena;
mod camera;
mod level_handling;
mod menu;
mod utils;
mod yoleck_ext;

use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use bevy_yoleck::prelude::YoleckSyncWithEditorState;

use self::arena::ArenaPlugin;
use self::camera::GameCamrePlugin;
use self::level_handling::{LevelHandlingPlugin, LevelProgress};
use self::menu::MenuPlugin;
use self::yoleck_ext::{AlignToGridPlugin, ResizeKnobsPlugin};

// Edit these:
const GAME_TITLE_FOR_MENU: &str = "Game Jam Template";
pub const ORG_AND_APP_NAMES: Option<(&str, &str)> = None;

pub struct GameMainPlugin {
    pub is_editor: bool,
    pub start_at_level: Option<String>,
}

impl Plugin for GameMainPlugin {
    fn build(&self, app: &mut App) {
        for schedule in [Update.intern(), FixedUpdate.intern()] {
            app.configure_sets(
                schedule,
                (
                    During::Menu.run_if(|state: Res<State<AppState>>| state.is_menu()),
                    During::Gameplay.run_if(in_state(AppState::Game)),
                ),
            );
        }
        app.insert_state(AppState::MainMenu);
        app.insert_resource(GameOverReason::Unset);
        app.add_systems(
            OnEnter(AppState::Game),
            GameOverReason::reset_when_gameplay_starts,
        );

        app.add_plugins(GameCamrePlugin);
        app.add_plugins(ArenaPlugin);

        if self.is_editor {
            app.add_plugins(YoleckSyncWithEditorState {
                when_editor: AppState::Editor,
                when_game: AppState::Game,
            });

            app.add_plugins(ResizeKnobsPlugin);
            app.add_plugins(AlignToGridPlugin); // only needed if AlignedToGrid is used
        } else {
            app.add_plugins((MenuPlugin, LevelHandlingPlugin));
            if let Some(start_at_level) = &self.start_at_level {
                let start_at_level = if start_at_level.ends_with(".yol") {
                    start_at_level.clone()
                } else {
                    format!("{}.yol", start_at_level)
                };
                app.add_systems(
                    Startup,
                    move |mut level_progress: ResMut<LevelProgress>,
                          mut app_state: ResMut<NextState<AppState>>| {
                        level_progress.current_level = Some(start_at_level.clone());
                        app_state.set(AppState::LoadLevel);
                    },
                );
            }
        }
    }
}

#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub enum During {
    Menu,
    Gameplay,
}

#[derive(States, Clone, Hash, Debug, PartialEq, Eq)]
pub enum AppState {
    MainMenu,
    PauseMenu,
    LevelSelectMenu,
    LoadLevel,
    Editor,
    Game,
    LevelCompleted,
    GameOver,
}

impl AppState {
    pub fn is_menu(&self) -> bool {
        match self {
            AppState::MainMenu => true,
            AppState::PauseMenu => true,
            AppState::LevelSelectMenu => true,
            AppState::LoadLevel => false,
            AppState::Editor => false,
            AppState::Game => false,
            AppState::LevelCompleted => false,
            AppState::GameOver => true,
        }
    }
}

#[derive(Resource)]
pub enum GameOverReason {
    Unset,
    PlayerFell,
}

impl GameOverReason {
    pub fn reason_text(&self) -> Option<String> {
        match self {
            GameOverReason::Unset => None,
            GameOverReason::PlayerFell => Some("player fell off the arena".to_owned()),
        }
    }
}

impl GameOverReason {
    fn reset_when_gameplay_starts(mut res: ResMut<Self>) {
        *res = Self::Unset;
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ActionForKbgp {
    Menu,
    RestartLevel,
    EndLevel,
}
