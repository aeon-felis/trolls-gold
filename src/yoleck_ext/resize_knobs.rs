use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol::prelude::*;

use crate::utils::CachedPbrMaker;

use super::AlignedToGrid;

pub struct ResizeKnobsPlugin;

impl Plugin for ResizeKnobsPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_edit_system(edit_scale_with_knobs);
    }
}

#[derive(Component)]
pub struct ResizeKnobs {
    pub axes: BVec3,
}

fn edit_scale_with_knobs(
    // TODO: handle rotation
    mut edit: YoleckEdit<(
        &ResizeKnobs,
        &mut Vpeol3dScale,
        &mut Vpeol3dPosition,
        Has<AlignedToGrid>,
    )>,
    mut pbr: CachedPbrMaker,
    mut knobs: YoleckKnobs,
) {
    let Ok((resize_knobs, mut vpeol_scale, mut vpeol_pos, aligned_to_grid)) = edit.single_mut()
    else {
        return;
    };

    let pbr = pbr.make_pbr_with(
        || Mesh::from(Cuboid::new(1.0, 1.0, 1.0)),
        || StandardMaterial::from_color(css::YELLOW_GREEN),
    );

    for i in 0..3 {
        let mut rotated_axes = Vec3::AXES;
        rotated_axes.rotate_right(i);
        let [u, v, w] = rotated_axes;
        for offset in [u + v, u - v, -u - v, -u + v] {
            let mut knob = knobs.knob(("scale-with-knobs", offset.as_ivec3()));
            let separation_from_surface = 0.4 * offset;
            knob.cmd.insert((
                Transform::from_translation(
                    vpeol_pos.0 + separation_from_surface + (0.5 * vpeol_scale.0) * offset,
                )
                .with_scale(w * 1.1 * vpeol_scale.0 + 0.4 * offset.abs()),
                pbr.clone(),
                VpeolDragPlane(InfinitePlane3d::new(w)),
            ));
            if let Some(drag) = knob.get_passed_data::<Vec3>() {
                let new_vec_to = (drag - vpeol_pos.0 - separation_from_surface) * 2.0;
                let new_size = new_vec_to * offset + w * vpeol_scale.0;
                let new_size =
                    Vec3::select(resize_knobs.axes, new_size, vpeol_scale.0).max(0.1 * Vec3::ONE);
                let new_size = if aligned_to_grid {
                    new_size.round().max(Vec3::ONE)
                } else {
                    // The math won't math without this
                    0.5 * (new_size + vpeol_scale.0)
                };
                let size_diff = new_size - vpeol_scale.0;
                vpeol_scale.0 = new_size;
                vpeol_pos.0 += 0.5 * offset * size_diff;
            }
        }
    }
}
