use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_rand::plugin::EntropyPlugin;
use bevy_rand::prelude::WyRand;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use trolls_gold::GameMainPlugin;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        // Wasm builds will check for meta files (that don't exist) if this isn't set.
        // This causes errors and even panics in web builds on itch.
        // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
        meta_check: AssetMetaCheck::Never,
        ..default()
    }));

    app.add_plugins((
        EntropyPlugin::<WyRand>::default(),
        RapierPhysicsPlugin::<NoUserData>::default(),
    ));

    app.add_plugins(GameMainPlugin);

    app.run()
}
