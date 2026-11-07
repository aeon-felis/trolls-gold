use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_tnua::builtins::TnuaBuiltinDash;
use bevy_tnua::prelude::*;
use ordered_float::OrderedFloat;

use crate::AppState;
use crate::player::IsPlayer;
use crate::protons::{GrabbingProton, IsProton};

pub struct PlayerControlPlugin;

impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TnuaControllerPlugin::<ControlScheme>::new(Update));
        app.add_systems(
            Update,
            (apply_controls, apply_grab).run_if(in_state(AppState::Game)),
        );
        app.add_input_context::<IsPlayer>();
    }
}

pub fn player_controls_for_scene() -> impl SceneList {
    bsn_list! {
        Action::<InputWalk>::new()
        Bindings [
            {preset::cardinal::arrows()}
            --
            {preset::cardinal::wasd_keys()}
            --
            {preset::axial::left_stick()}
            --
            {preset::cardinal::dpad()}
        ]
        --
        Action::<InputGrab>::new()
        Bindings [
            Binding(KeyCode::Space)
            --
            Binding(GamepadButton::South)
        ]
        --
        Action::<InputDash>::new()
        Bindings [
            Binding(KeyCode::ShiftLeft)
            --
            Binding(KeyCode::ShiftRight)
            --
            Binding(GamepadButton::East)
        ]
    }
}

#[derive(InputAction)]
#[action_output(Vec2)]
struct InputWalk;

#[derive(InputAction)]
#[action_output(bool)]
struct InputGrab;

#[derive(InputAction)]
#[action_output(bool)]
struct InputDash;

#[derive(TnuaScheme)]
#[scheme(basis=TnuaBuiltinWalk)]
pub enum ControlScheme {
    Dash(TnuaBuiltinDash),
}

fn apply_controls(
    mut query: Query<(
        &mut TnuaController<ControlScheme>,
        &Actions<IsPlayer>,
        &GlobalTransform,
    )>,
    walk: Query<&Action<InputWalk>>,
    dash: Query<&Action<InputDash>>,
    proton_locations: Query<&GlobalTransform, With<IsProton>>,
) {
    for (mut controller, actions, transform) in query.iter_mut() {
        controller.initiate_action_feeding();

        let Some(walk) = walk.iter_many(actions).matched().next() else {
            continue;
        };

        let Some(dash) = dash.iter_many(actions).matched().next() else {
            continue;
        };

        let walk_input_vector = walk.clamp_length_max(1.0);
        // TODO: rotate based on camera
        controller.basis.desired_motion = Vec3::new(walk_input_vector.x, 0.0, -walk_input_vector.y);

        if **dash {
            if let Ok(input_direction) = Dir3::new(controller.basis.desired_motion) {
                let position = transform.translation();

                let dash_direction = if let Some((direction_to_targeted_proton, _, _)) =
                    proton_locations
                        .iter()
                        .filter_map(|proton_transform| {
                            let vector = proton_transform.translation() - position;
                            let (dir, dist) = Dir3::new_and_length(vector).ok()?;
                            let dot = dir.dot(*input_direction);
                            if dot <= 0.25 {
                                None
                            } else {
                                Some((dir, dist, dot))
                            }
                        })
                        .max_by_key(|(_dir, dist, dot)| OrderedFloat((1.0 - dot) * (-dist).exp()))
                {
                    direction_to_targeted_proton
                } else {
                    input_direction
                };
                controller.action(ControlScheme::Dash(TnuaBuiltinDash {
                    displacement: *dash_direction,
                    desired_forward: Some(dash_direction),
                    allow_in_air: true,
                }));
            } else {
                controller.action_fake(ControlSchemeActionDiscriminant::Dash);
            }
        }
    }
}

fn apply_grab(
    query: Query<(Entity, &Actions<IsPlayer>, Has<GrabbingProton>)>,
    grab: Query<&Action<InputGrab>>,
    proton_check: Query<(), With<IsProton>>,
    rapier_context: ReadRapierContext,
    mut commands: Commands,
) {
    let rapier_context = rapier_context.single().unwrap();
    for (player, actions, grabbing) in query.iter() {
        let Some(grab) = grab.iter_many(actions).matched().next() else {
            continue;
        };
        match (**grab, grabbing) {
            (true, true) | (false, false) => {}
            (false, true) => {
                commands.entity(player).remove::<GrabbingProton>();
            }
            (true, false) => {
                for entity in rapier_context
                    .contact_pairs_with(player)
                    .flat_map(|contact_pair| [contact_pair.collider1(), contact_pair.collider2()])
                    .filter_map(|entity| entity.filter(|entity| *entity != player))
                {
                    if proton_check.contains(entity) {
                        commands.entity(player).insert(GrabbingProton(entity));
                        break;
                    }
                }
            }
        }
    }
}
