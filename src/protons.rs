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
        app.insert_resource(StrongNuclearForce::Active);
        app.add_systems(OnEnter(AppState::InitScene), spawn_protons);
        app.add_systems(
            FixedUpdate,
            apply_strong_nuclear_force.run_if(in_state(AppState::Game)),
        );
        app.add_observer(shake_strong_nuclear_force);
    }
}

const PROTON_RADIUS: f32 = 0.5;

#[derive(Component, Default, Clone)]
pub struct IsProton;

#[derive(Resource)]
enum StrongNuclearForce {
    Active,
    Shaken(Timer),
}

#[derive(Event)]
pub struct ShakeStrongNuclearForce;

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
            Velocity
            GravityScale(0.0)
            CollisionGroups {
                memberships: PARTICLE_COLLISION_GROUP,
            }
        });
    }
}

fn shake_strong_nuclear_force(_: On<ShakeStrongNuclearForce>, 
    mut strong_nuclear_force: ResMut<StrongNuclearForce>,
) {
    *strong_nuclear_force = StrongNuclearForce::Shaken(Timer::from_seconds(5.0, TimerMode::Once));
}

fn apply_strong_nuclear_force(
    time: Res<Time>,
    mut strong_nuclear_force: ResMut<StrongNuclearForce>,
    mut protons_query: Query<(Entity, &GlobalTransform, &mut ExternalForce), With<IsProton>>,
    other_protons_query: Query<(Entity, &GlobalTransform), With<IsProton>>,
    mut protons_velocities: Query<&mut Velocity, With<IsProton>>,
) {
    let is_shaken = if let StrongNuclearForce::Shaken(timer) = strong_nuclear_force.as_mut() {
        if timer.tick(time.delta()).is_finished() {
            *strong_nuclear_force = StrongNuclearForce::Active;
            false
        } else {
            true
        }
    } else {
        false
    };

    if is_shaken {
        protons_velocities.par_iter_mut().for_each(|mut velocity| {
            velocity.linear = velocity.linear.clamp_length_max(4.0);
        });
        return;
    }

    // TODO: exclude from the center protons separate from the others
    let center_direction = Dir3::new(
        other_protons_query
            .iter()
            .map(|(_, transform)| transform.translation())
            .sum::<Vec3>(),
    )
    .ok();
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
            if let Some(center_direction) = center_direction {
                force.force -= *center_direction;
            }
        })
}
