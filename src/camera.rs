use bevy::prelude::*;

pub struct GameCamrePlugin;

impl Plugin for GameCamrePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_scene_list(bsn_list! {
        Camera3d
        Transform::from_xyz(0.0, 20.0, 80.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y)
        --
        DirectionalLight {
            color: Color::WHITE
        }
        Transform::from_xyz(0.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::NEG_Z)
        --
        DirectionalLight {
            color: Color::WHITE
        }
        Transform::from_xyz(0.0, -100.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z)
    });
}
