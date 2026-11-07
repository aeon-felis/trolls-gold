use bevy::color::palettes::css;
use bevy::math::ops::exp;
use bevy::platform::collections::HashMap;
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
            (
                (apply_strong_nuclear_force, apply_proton_grabbing)
                    .run_if(in_state(AppState::Game)),
                update_protons_clustering.run_if(does_protons_clustering_need_update),
            ),
        );
    }
}

const PROTON_RADIUS: f32 = 0.5;

#[derive(Component, Default, Clone)]
pub struct IsProton;

#[derive(Component)]
pub struct GrabbingProton(pub Entity);

#[derive(Component, Default, Clone, Debug)]
pub struct NucleusAffinity {
    cluster_index: usize,
    cluster_size: usize,
    #[allow(unused)]
    num_neighbors: usize,
}

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
            NucleusAffinity
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
            ExternalImpulse
            GravityScale(0.0)
            CollisionGroups {
                memberships: PARTICLE_COLLISION_GROUP,
            }
            ActiveEvents::COLLISION_EVENTS
        });
    }
}

fn apply_strong_nuclear_force(
    mut protons_query: Query<
        (
            Entity,
            &GlobalTransform,
            &NucleusAffinity,
            &mut ExternalForce,
        ),
        With<IsProton>,
    >,
    other_protons_query: Query<(Entity, &GlobalTransform, &NucleusAffinity), With<IsProton>>,
) {
    let num_clusters = other_protons_query
        .iter()
        .map(|(_, _, nucleus)| nucleus.cluster_index + 1)
        .max()
        .unwrap_or(0);
    let mut nucli_centers = vec![Vec3::ZERO; num_clusters];

    for (_, transform, nucleus) in other_protons_query {
        // NOTE: not dividing by cluster size because we only need the direction
        nucli_centers[nucleus.cluster_index] += transform.translation();
    }
    let center_directions: Vec<Option<Dir3>> = nucli_centers
        .into_iter()
        .map(Dir3::new)
        .map(Result::ok)
        .collect();

    protons_query
        .par_iter_mut()
        .for_each(|(entity, transform, nucleus, mut force)| {
            let position = transform.translation();
            force.force = other_protons_query
                .iter()
                .filter_map(|(other_entity, other_transform, _)| {
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
            if 1 < nucleus.cluster_size
                && let Some(center_direction) = center_directions[nucleus.cluster_index]
            {
                force.force -= *center_direction;
            }
        })
}

fn apply_proton_grabbing(
    mut grabber_query: Query<
        (&GrabbingProton, &GlobalTransform, &mut ExternalImpulse),
        Without<IsProton>,
    >,
    mut protons_query: Query<(&GlobalTransform, &mut ExternalImpulse), With<IsProton>>,
) {
    for (GrabbingProton(grabbed_proton), grabber_transform, mut grabber_impulse) in
        grabber_query.iter_mut()
    {
        let Ok((proton_transform, mut proton_impulse)) = protons_query.get_mut(*grabbed_proton)
        else {
            continue;
        };

        let vec_to_proton = proton_transform.translation() - grabber_transform.translation();
        let impulse_vec = vec_to_proton;

        grabber_impulse.impulse = impulse_vec;
        proton_impulse.impulse = -impulse_vec;
    }
}

fn does_protons_clustering_need_update(
    mut reader: MessageReader<CollisionEvent>,
    is_proton_query: Query<(), With<IsProton>>,
) -> bool {
    if reader.read().any(|event| {
        let (CollisionEvent::Started(e1, e2, _) | CollisionEvent::Stopped(e1, e2, _)) = event;
        is_proton_query.contains(*e1) && is_proton_query.contains(*e2)
    }) {
        // Don't let collision events queue and affect later frames
        for _ in reader.read() {}
        true
    } else {
        false
    }
}

fn update_protons_clustering(
    mut query: Query<(Entity, &mut NucleusAffinity), With<IsProton>>,
    rapier_context: ReadRapierContext,
) {
    let rapier_context = rapier_context.single().unwrap();
    let mut cluster_sizes: Vec<usize> = Default::default();

    struct ClusteringEntry {
        cluster_index: usize,
        num_neighbors: usize,
    }

    let mut clustering: HashMap<Entity, ClusteringEntry> = Default::default();
    let mut to_check: Vec<Entity> = Default::default();
    for (first_proton_in_cluster, _) in query.iter() {
        if clustering.contains_key(&first_proton_in_cluster) {
            continue;
        }
        to_check.push(first_proton_in_cluster);
        let cluster_index = cluster_sizes.len();
        let mut cluster_size = 0;
        while let Some(proton) = to_check.pop() {
            if clustering.contains_key(&proton) {
                continue;
            }
            cluster_size += 1;
            let prev_len = to_check.len();
            to_check.extend(
                rapier_context
                    .contact_pairs_with(proton)
                    .flat_map(|contact_pair| [contact_pair.collider1(), contact_pair.collider2()])
                    .filter_map(|entity| entity.filter(|entity| *entity != proton)),
            );
            clustering.insert(
                proton,
                ClusteringEntry {
                    cluster_index,
                    num_neighbors: to_check.len() - prev_len,
                },
            );
        }
        cluster_sizes.push(cluster_size);
    }

    for (proton, mut nucleus) in query.iter_mut() {
        let ClusteringEntry {
            cluster_index,
            num_neighbors,
        } = clustering[&proton];
        *nucleus = NucleusAffinity {
            cluster_index,
            cluster_size: cluster_sizes[cluster_index],
            num_neighbors,
        };
    }
}
