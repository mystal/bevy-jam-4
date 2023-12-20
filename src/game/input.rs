use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<PlayerInput>()
            .add_systems(PreUpdate, (
                read_player_input,
            ));
    }
}

#[derive(Default, Component, Reflect)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub aim: Vec2,
    pub aim_device: AimDevice,
    pub shoot: bool,
    pub next_weapon: bool,
    pub prev_weapon: bool,
    pub reset_game: bool,
}

// Taken from:
// https://bevy-cheatbook.github.io/cookbook/cursor2world.html#2d-games
pub fn get_mouse_world_pos(
    window_q: &Query<&Window, With<PrimaryWindow>>,
    camera_q: &Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let (camera, camera_transform) = camera_q.single();
    let window = window_q.single();

    // Check if the cursor is inside the window and get its position.
    window.cursor_position()
        .and_then(|cursor_pos| camera.viewport_to_world_2d(camera_transform, cursor_pos))
}

#[derive(Clone, Copy, Default, Reflect)]
pub enum AimDevice {
    #[default]
    None,
    Gamepad,
    Mouse(Vec2),
}

pub fn read_player_input(
    keys: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut cursor_moved: EventReader<CursorMoved>,
    gamepads: Res<Gamepads>,
    pad_buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut egui_ctx: EguiContexts,
    mut player_q: Query<(&mut PlayerInput, &GlobalTransform)>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    primary_window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok((mut input, player_transform)) = player_q.get_single_mut() else {
        return;
    };

    let mut movement = Vec2::ZERO;
    let mut aim = Vec2::ZERO;
    let mut aim_device = input.aim_device;
    let mut shoot = false;
    let mut reset_game = false;

    // Read input from gamepad.
    if let Some(gamepad) = gamepads.iter().next() {
        // Movement
        let move_x = GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX);
        let move_y = GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY);
        if let (Some(x), Some(y)) = (axes.get(move_x), axes.get(move_y)) {
            let tmp = Vec2::new(x, y);
            // TODO: See if we can configure the deadzone using Bevy's APIs.
            if tmp.length() > 0.1 {
                movement = tmp;
            }
        }

        // Aim
        let aim_x = GamepadAxis::new(gamepad, GamepadAxisType::RightStickX);
        let aim_y = GamepadAxis::new(gamepad, GamepadAxisType::RightStickY);
        if let (Some(x), Some(y)) = (axes.get(aim_x), axes.get(aim_y)) {
            let tmp = Vec2::new(x, y);
            // TODO: See if we can configure the deadzone using Bevy's APIs.
            if tmp.length() > 0.1 {
                aim = tmp;
                aim_device = AimDevice::Gamepad;
            } else {
                aim_device = AimDevice::None;
            }
        }

        // Shoot
        let shoot_button = GamepadButton::new(gamepad, GamepadButtonType::RightTrigger2);
        shoot |= pad_buttons.pressed(shoot_button);

        let reset_button = GamepadButton::new(gamepad, GamepadButtonType::Start);
        reset_game |= pad_buttons.pressed(reset_button);
    }

    // Read input from mouse/keyboard.
    // Movement
    if movement == Vec2::ZERO && !egui_ctx.ctx_mut().wants_keyboard_input() {
        let x = (keys.pressed(KeyCode::D) as i8 - keys.pressed(KeyCode::A) as i8) as f32;
        let y = (keys.pressed(KeyCode::W) as i8 - keys.pressed(KeyCode::S) as i8) as f32;
        movement = Vec2::new(x, y).normalize_or_zero();
    }

    // Aim
    let mouse_moved = cursor_moved.read().count() > 0;
    // Try to use mouse for aim if the gamepad isn't being used and the mouse moved or we were
    // already using the mouse.
    if aim == Vec2::ZERO && (mouse_moved || matches!(input.aim_device, AimDevice::Mouse(_))) {
        if let Some(pos) = get_mouse_world_pos(&primary_window_q, &camera_q) {
            aim = (pos - player_transform.translation().truncate()).normalize_or_zero();
            aim_device = AimDevice::Mouse(pos);
        }
    }

    // Shoot
    shoot |= keys.pressed(KeyCode::Space) && !egui_ctx.ctx_mut().wants_keyboard_input();

    reset_game |= keys.just_pressed(KeyCode::Space) && !egui_ctx.ctx_mut().wants_keyboard_input();

    // Store results in player input component.
    input.movement = movement;
    input.aim = aim;
    input.aim_device = aim_device;
    input.shoot = shoot;
    input.reset_game = reset_game;
}
