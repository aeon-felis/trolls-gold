use bevy::prelude::*;

use crate::player::IsPlayer;

pub struct GameCamrePlugin;

impl Plugin for GameCamrePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, position_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_scene_list(bsn_list!{
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

fn position_camera(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    player_query: Query<&GlobalTransform, With<IsPlayer>>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else { return };
    let Ok(player_transform) = player_query.single() else { return };
    let mut player_postion = player_transform.translation();
    player_postion.y = 0.0;
    let Ok((player_direction, player_distance)) = Dir3::new_and_length(player_postion) else { return };
    *camera_transform = Transform::from_translation(player_direction * (1.5 * player_distance + 20.0) + Vec3::Y * 16.0).looking_at(Vec3::ZERO, Vec3::Y);
}
