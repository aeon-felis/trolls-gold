use bevy::prelude::*;
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol_3d::*;

#[derive(Component)]
pub struct AlignedToGrid;

pub struct AlignToGridPlugin;

impl Plugin for AlignToGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_edit_system(align_to_grid_while_editing);
    }
}

fn align_to_grid_while_editing(
    mut edit: YoleckEdit<(&mut Vpeol3dPosition, Option<&mut Vpeol3dScale>), With<AlignedToGrid>>,
) {
    for (mut vpeol_pos, vpeol_scale) in edit.iter_matching_mut() {
        if let Some(mut vpeol_scale) = vpeol_scale {
            for (pos, scale) in vpeol_pos.0.as_mut().iter_mut().zip(vpeol_scale.0.as_mut()) {
                *scale = scale.round();
                if (*scale as isize) % 2 == 0 {
                    *pos = (*pos - 0.5).round() + 0.5;
                } else {
                    *pos = pos.round();
                }
            }
        } else {
            vpeol_pos.0 = vpeol_pos.0.round();
        }
    }
}
