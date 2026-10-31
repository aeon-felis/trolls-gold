use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol::prelude::*;

use crate::utils::CachedPbrMaker;
use crate::yoleck_ext::{AlignedToGrid, ResizeKnobs};

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        // Only one of these two `YoleckEntityType`s is actually needed - pick the one you want and
        // delete the irrelevant functions/types from this file.
        app.add_yoleck_entity_type({
            // This platform can freely be placed, resized, and rotated.
            YoleckEntityType::new("BoxPlatform")
                .with::<Vpeol3dPosition>()
                .with::<Vpeol3dScale>()
                .with::<Vpeol3dRotation>()
                .insert_on_init_during_editor(|| ResizeKnobs { axes: BVec3::TRUE })
                .insert_on_init(|| IsPlatform)
                .insert_on_init(|| SolidBox)
        });
        app.add_yoleck_entity_type({
            // This platform cannot be rotated, and its position and size are aligned to a grid.
            YoleckEntityType::new("BricksPlatform")
                .with::<Vpeol3dPosition>()
                .with::<Vpeol3dScale>()
                .insert_on_init_during_editor(|| (ResizeKnobs { axes: BVec3::TRUE }, AlignedToGrid))
                .insert_on_init(|| (IsPlatform, MadeOfBricks))
        });

        app.add_systems(
            YoleckSchedule::Populate,
            (
                // Here, too, remove the one you don't use.
                populate_box_platform,
                populate_bricks_platform,
            ),
        );
    }
}

/// Used by other systems to know that this entity is a platform
#[derive(Component)]
pub struct IsPlatform;

// These two components are used to distinguish between the two types of platforms the plugin
// offers. `AlignedToGrid` cannot be used for this purpose since it only gets inserted in editor
// mode.
#[derive(Component)]
struct SolidBox;
#[derive(Component)]
struct MadeOfBricks;

fn populate_box_platform(
    mut populate: YoleckPopulate<(), (With<IsPlatform>, With<SolidBox>)>,
    mut pbr: CachedPbrMaker,
) {
    populate.populate(|ctx, mut cmd, _| {
        if ctx.is_first_time() {
            cmd.insert(pbr.make_pbr_with(
                || Mesh::from(Cuboid::new(1.0, 1.0, 1.0)),
                || StandardMaterial::from_color(css::GRAY),
            ));
        }
    });
}

#[derive(Component)]
struct CurrentSizeInBlocks(UVec3);

fn populate_bricks_platform(
    mut populate: YoleckPopulate<
        (&Vpeol3dScale, Option<&CurrentSizeInBlocks>),
        (With<IsPlatform>, With<MadeOfBricks>),
    >,
    mut pbr: CachedPbrMaker,
) {
    populate.populate(|ctx, mut cmd, (scale, current_size_in_blocks)| {
        let size_in_blocks =
            UVec3::from_array(scale.0.to_array().map(|c| (c.floor().abs() as u32).max(1)));

        if let Some(CurrentSizeInBlocks(current_size_in_blocks)) = current_size_in_blocks {
            if *current_size_in_blocks == size_in_blocks {
                return;
            } else {
                cmd.despawn_children();
            }
        }

        let pbr = pbr.make_pbr_with(
            || Mesh::from(Cuboid::new(0.9, 0.9, 0.9)),
            || StandardMaterial::from_color(css::DARK_RED),
        );

        let offset = -0.5 * (scale.0 - Vec3::ONE);

        if ctx.is_in_editor() {
            cmd.insert((
                VpeolWillContainClickableChildren,
                CurrentSizeInBlocks(size_in_blocks),
            ));
        }

        let reverse_scale = scale.0.map(|c| 1.0 / c);

        cmd.with_children(|commands| {
            for x in 0..size_in_blocks.x {
                for y in 0..size_in_blocks.y {
                    for z in 0..size_in_blocks.z {
                        commands.spawn((
                            Transform {
                                translation: (offset + Vec3::new(x as f32, y as f32, z as f32))
                                    * reverse_scale,
                                rotation: Quat::IDENTITY,
                                scale: reverse_scale,
                            },
                            pbr.clone(),
                        ));
                    }
                }
            }
            commands.spawn(());
        });
    });
}
