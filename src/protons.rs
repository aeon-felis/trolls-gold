use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy_rand::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::RngExt;

use crate::AppState;

pub struct ProtonsPlugin;

impl Plugin for ProtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InitScene), setup_protons);
    }
}

const PROTON_RADIUS: f32 = 0.5;

fn setup_protons(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    let mesh = meshes.add(Sphere::new(PROTON_RADIUS));
    let material = materials.add(Color::from(css::RED));

    for _ in 0..78 {
        let position = Vec3::new(
            rng.random_range(-1.0..=1.0),
            rng.random_range(-1.0..=1.0),
            rng.random_range(-1.0..=1.0),
        );
        commands.spawn_scene(bsn! {
            Visibility
            Transform {
                translation: position,
            }
            Mesh3d({mesh.clone()})
            MeshMaterial3d<StandardMaterial>({material.clone()})
            RigidBody::Dynamic
            Collider::ball(PROTON_RADIUS)
            GravityScale(0.0)
        });
    }
}
