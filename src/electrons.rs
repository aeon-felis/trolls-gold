use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy_rand::global::GlobalRng;
use bevy_rand::prelude::WyRand;
use bevy_rapier3d::prelude::*;
use rand::RngExt;

use crate::{AppState, ClearableObject};

pub struct ElectronsPlugin;

impl Plugin for ElectronsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InitScene), spawn_electrons);
        app.add_systems(FixedUpdate, move_electrons.run_if(in_state(AppState::Game)));
    }
}

const ELECTRON_RADIUS: f32 = 0.3;

#[derive(Component, Default, Clone)]
pub struct IsElectron {
    angle: f32,
    orbit_radius: f32,
    orbit_speed: f32,
}

fn spawn_electrons(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    let mesh = meshes.add(Sphere::new(ELECTRON_RADIUS));
    let material = materials.add(Color::from(css::BLUE));

    for i in 0..78 {
        let orbit_radius = i as f32 * 0.5 + 8.0;
        let angle = rng.random_range(0.0..2.0 * std::f32::consts::PI);
        let position = Quat::from_rotation_y(angle) * (orbit_radius * Vec3::X);
        let orbit_speed: f32 = rng.random_range(50.0..60.0) / orbit_radius;
        commands.spawn_scene(bsn! {
            IsElectron {
                angle
                orbit_radius,
                orbit_speed,
            }
            ClearableObject
            Visibility
            Transform {
                translation: position,
            }
            Mesh3d({mesh.clone()})
            MeshMaterial3d<StandardMaterial>({material.clone()})
            RigidBody::KinematicPositionBased
            Collider::ball(ELECTRON_RADIUS)
            GravityScale(0.0)
            ActiveEvents::COLLISION_EVENTS
        });
    }
}

fn move_electrons(time: Res<Time>, mut query: Query<(&mut IsElectron, &mut Transform)>) {
    query
        .par_iter_mut()
        .for_each(|(mut electron, mut transform)| {
            electron.angle += electron.orbit_speed * time.delta_secs();
            electron.angle %= 2.0 * std::f32::consts::PI;
            transform.translation =
                Quat::from_rotation_y(electron.angle) * Vec3::X * electron.orbit_radius;
        });
}
