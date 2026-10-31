use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_egui_kbgp::prelude::*;
use bevy_pkv::PkvStore;
use bevy_yoleck::vpeol_3d::{Vpeol3dPluginForEditor, Vpeol3dPluginForGame};
use bevy_yoleck::{YoleckPluginForEditor, YoleckPluginForGame};
use clap::Parser;
use my_game_jam_template::{ActionForKbgp, GameMainPlugin, ORG_AND_APP_NAMES};

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    editor: bool,
    #[clap(long)]
    level: Option<String>,
}

fn main() -> AppExit {
    let args = Args::parse();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        // Wasm builds will check for meta files (that don't exist) if this isn't set.
        // This causes errors and even panics in web builds on itch.
        // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
        meta_check: AssetMetaCheck::Never,
        ..default()
    }));

    // Egui is required both for Yoleck and for the menu system
    app.add_plugins(EguiPlugin::default());

    if let Some((org_name, app_name)) = ORG_AND_APP_NAMES {
        app.insert_resource(PkvStore::new(org_name, app_name));
    }

    if args.editor {
        app.add_plugins((YoleckPluginForEditor, Vpeol3dPluginForEditor::topdown()));
    } else {
        app.add_plugins((YoleckPluginForGame, Vpeol3dPluginForGame));
        app.add_plugins(KbgpPlugin);
        app.insert_resource(KbgpSettings {
            disable_default_navigation: true,
            disable_default_activation: true,
            prevent_loss_of_focus: true,
            focus_on_mouse_movement: true,
            allow_keyboard: true,
            allow_mouse_buttons: true,
            allow_mouse_wheel: false,
            allow_mouse_wheel_sideways: false,
            allow_gamepads: true,
            bindings: {
                let binding = KbgpNavBindings::default()
                    .with_wasd_navigation()
                    .with_key(KeyCode::Escape, KbgpNavCommand::user(ActionForKbgp::Menu))
                    .with_key(
                        KeyCode::Backspace,
                        KbgpNavCommand::user(ActionForKbgp::RestartLevel),
                    )
                    .with_key(KeyCode::Enter, KbgpNavCommand::Click)
                    .with_key(KeyCode::NumpadEnter, KbgpNavCommand::Click)
                    .with_key(KeyCode::Space, KbgpNavCommand::Click)
                    .with_gamepad_button(
                        GamepadButton::Start,
                        KbgpNavCommand::user(ActionForKbgp::Menu),
                    )
                    .with_gamepad_button(
                        GamepadButton::Select,
                        KbgpNavCommand::user(ActionForKbgp::RestartLevel),
                    )
                    .with_key(KeyCode::KeyJ, KbgpNavCommand::Click);

                // This is useful for the template which does not implement any actual level
                // completion conditions, but actual games should just remove it.
                #[cfg(debug_assertions)]
                let binding =
                    binding.with_key(KeyCode::End, KbgpNavCommand::user(ActionForKbgp::EndLevel));

                #[allow(clippy::let_and_return)]
                binding
            },
        });
    }

    app.add_plugins(GameMainPlugin {
        is_editor: args.editor,
        start_at_level: args.level,
    });

    app.run()
}
