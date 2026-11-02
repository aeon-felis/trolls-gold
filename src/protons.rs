use bevy::color::palettes::css;
use bevy::math::ops::exp;
use bevy::prelude::*;
use bevy_rand::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::RngExt;

use crate::{AppState, ClearableObject, PARTICLE_COLLISION_GROUP};

pub struct ProtonsPlugin;

impl Plugin for ProtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InitScene), spawn_protons);
        app.add_systems(
            FixedUpdate,
            apply_strong_nuclear_force.run_if(in_state(AppState::Game)),
        );
    }
}

const PROTON_RADIUS: f32 = 0.5;

#[derive(Component, Default, Clone)]
pub struct IsProton;

fn spawn_protons(
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
            IsProton
            ClearableObject
            Visibility
            Transform {
                translation: position,
            }
            Mesh3d({mesh.clone()})
            MeshMaterial3d<StandardMaterial>({material.clone()})
            RigidBody::Dynamic
            Collider::ball(PROTON_RADIUS)
            ExternalForce
            GravityScale(0.0)
            CollisionGroups {
                memberships: PARTICLE_COLLISION_GROUP,
            }
        });
    }
}

fn apply_strong_nuclear_force(
    mut protons_query: Query<(Entity, &GlobalTransform, &mut ExternalForce), With<IsProton>>,
    other_protons_query: Query<(Entity, &GlobalTransform), With<IsProton>>,
) {
    protons_query
        .par_iter_mut()
        .for_each(|(entity, transform, mut force)| {
            let position = transform.translation();
            force.force = other_protons_query
                .iter()
                .filter_map(|(other_entity, other_transform)| {
                    if entity == other_entity {
                        return None;
                    }
                    let other_position = other_transform.translation();
                    let vector = other_position - position;
                    let (dir, dist) = Dir3::new_and_length(vector).ok()?;
                    let strength = 176.0 * exp(-3.0 * dist) / dist;
                    Some(dir * strength)
                })
                .sum::<Vec3>();
        })
}
