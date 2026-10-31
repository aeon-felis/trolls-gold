use bevy::prelude::*;
use bevy_yoleck::vpeol::prelude::*;

pub struct GameCamrePlugin;

impl Plugin for GameCamrePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 16.0, 40.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        VpeolCameraState::default(),
        Vpeol3dCameraControl::topdown(),
    ));
}
