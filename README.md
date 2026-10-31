# My Personal Template for Game Jams

This is forked from the [official template](https://github.com/bevyengine/bevy_github_ci_template), and I'm adding the things I usually use and are a pain to set up.

## Plugins used in this template

Bevy plugins by yours truly:

* [bevy-yoleck](https://github.com/idanarye/bevy-yoleck) - level editing
* [bevy-egui-kbgp](https://github.com/idanarye/bevy-egui-kbgp) - menu navigation

Bevy plugins by other people:

* [bevy_egui](https://github.com/vladbat00/bevy_egui) - UI for the menu (also used by Yoleck for the level editing UI)
* [bevy_pkv](https://github.com/johanhelsing/bevy_pkv) - save/load level progress

## Stuff you need to edit

* `package.name` in `Cargo.toml` - goes without saying.
* `env` variables in `.github/workflows/release.yaml` - see their comments or refer to the README in [original template](https://github.com/bevyengine/bevy_github_ci_template).
* Global consts at `src/lib.rs`:
  * `GAME_TITLE_FOR_MENU` - exactly what it says on the tin
  * `ORG_AND_APP_NAMES` - sets the namespace under which bevy_pkv saves the level progress data
* Probably want to delete the `KeyCode::End` binding in `src/main.rs` since it allows skipping a level.

## Yoleck Level Editing

To launch in level editor mode:

```bash
cargo run -- --editor
```

To start the game at a specific level, skipping both the main menu and the level progress (meaning you can play levels that weren't reached through gameplay):
```bash
cargo run -- --level <Level-Name-Without-.yol-Suffix>
```

### Arena (platforms)

`src/arena.rs` defines two types of platforms you can add in the level editor:

* `BoxPlatform` - which you can freely resize. It scales the transform, so if you use a texture it'd get messed up.
* `BricksPlatform` - both its location and size are snapped to a grid, using a mechanism implemented in `src/yoleck_ext/align_to_grid.rs`. Instead of a single PBR, its body is combosed of many identical objects (bricks) so it can properly deal with textured models.

Both can be resised in the level editor by dragging the green borders that appear when they are selected. This mechanism is implemented in `src/yoleck_ext/resize_knobs.rs`.

Actual games probably only need one of them, so you can delete the other.

The `populate_...` systems in the `Arena` create their PBR - edit these to create your own graphics.

### Main plane for level editing

This template adds Yoleck with [Vpeol 3D](https://docs.rs/bevy-yoleck/latest/bevy_yoleck/vpeol_3d/index.html) - a module bundled within Yoleck that adds basic support for editing levels in 3D games. But 3D here refers purley to the graphics - it is very possible to create a game with 3D graphics and 2D gameplay, either by preventing any verticality (no jumping, no falling, no ladders/stairs) or by preventing the characters from using the Z axis.

By default, this template configures Vpeol 3D for a top-down game. This means that, during level editing:

* When the camera is moved with WASD, it'll move along the XZ plane. The mouse wheel can be used to move along the Y axis.
* When an entity is dragged with the mouse's left button (on the entity itself - not on any of the knobs) it'll move along the XZ plane.

If you are making a side-scroller, this is very inconvenient. You want to use the XY plane instead. To do that:

* In `src/main.rs`, change `Vpeol3dPluginForEditor::topdown()` to `Vpeol3dPluginForEditor::sidescroller()`.
* In `src/camera.rs`, change `Vpeol3dCameraControl::topdown()` to `Vpeol3dCameraControl::sidescroller()`.

There is also `Vpeol3dCameraControl::fps()` for FPS camera (move with WASD, up/down with Q/E, and look around by dragging the mouse with the right button pressed). If you use that, it's probably good idea to leave `Vpeol3dPluginForEditor` as top-down (this affects dragging objects around) since with the FPS's camera's free rotation the XY plane is not very meaningful (you can still move objects along the Y axis using the gizmo)

## Menu

The menu is defined in `src/menu.rs`. The main part is s `chain()` of systems running in the `EguiPrimaryContextPass` - these systems draw the menu. To add a new section to the menu just add a system to that chain and have it use `ResMut<FrameUi>` to get access to egui.

## Menu Navigation

Look for `KbgpNavBindings` in `src/main.rs`.

## Level Progress

Implemented in `src/level_handling.rs`.

When the player completed the level, set `ResMut<NextState<AppState>>` to `AppState::LevelCompleted` - this will open the level selection menu with the next level unlocked. Since this template does not have any gameplay condition for finishing the levels, it adds a binding to the `End` key for testing level completion.

If the player dies or otherwise fails a level:

* Set `ResMut<NextState<AppState>>` to `AppState::GameOver`.
* Set `ResMut<GameOverReason>` to the reason why the player lost (edit `GameOverReason` to have the reasons relevant to your game)
