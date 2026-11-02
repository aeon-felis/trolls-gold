use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::player::PLAYER_HEIGHT;
use crate::{ARENA_COLLISION_GROUP, PLAYER_COLLISION_GROUP};

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_arena);
    }
}

fn setup_arena(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        #Floor
        RigidBody::Fixed
        Collider::halfspace(Vec3::Y).unwrap()
        Transform::from_xyz(0.0, -PLAYER_HEIGHT, 0.0)
        CollisionGroups {
            memberships: ARENA_COLLISION_GROUP,
            filters: PLAYER_COLLISION_GROUP,
        }
    });
}
