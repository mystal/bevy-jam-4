use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::{AudioInstance, AudioSource};
use bevy_rapier2d::render::{DebugRenderContext, RapierDebugRenderPlugin};

use crate::{
    AppState,
    // assets::AudioAssets,
    // enemies::spawner::Spawner,
    // game::{Bgm, GameTimers},
    game::input,
    // player::{self, PlayerInput},
    // weapons::{Weapon, WeaponChoice},
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                WorldInspectorPlugin::default().run_if(show_world_inspector),
                RapierDebugRenderPlugin::default().disabled(),
            ))

            .insert_resource(DebugState::default())
            // Run these before game player input because wants_pointer_input will return false
            // otherwise.
            .add_systems(Update, (
                debug_ui.run_if(debug_ui_enabled),
                toggle_debug_ui,
                toggle_physics_debug_render,
            ).before(input::read_player_input))
            .add_systems(Last, update_mouse_cursor);
    }
}

#[derive(Default, Resource)]
struct DebugState {
    enabled: bool,
    show_world_inspector: bool,
}

fn debug_ui_enabled(
    debug_ui: Res<DebugState>,
) -> bool {
    debug_ui.enabled
}

fn show_world_inspector(
    debug_ui: Res<DebugState>,
) -> bool {
    debug_ui.enabled && debug_ui.show_world_inspector
}

fn debug_ui(
    mut debug_state: ResMut<DebugState>,
    mut debug_physics_ctx: ResMut<DebugRenderContext>,
    mut egui_ctx: EguiContexts,
) {
    let ctx = egui_ctx.ctx_mut();

    egui::TopBottomPanel::top("debug_panel")
        .show(ctx, |ui| {
            // NOTE: An egui bug makes clicking on the menu bar not report wants_pointer_input,
            // which means it'll register as a click in game.
            // https://github.com/emilk/egui/issues/2606
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Debug", |ui| {
                    ui.checkbox(&mut debug_state.show_world_inspector, "World Inspector");
                    ui.checkbox(&mut debug_physics_ctx.enabled, "Debug Physics Render");
                });
            });
        });
}


fn update_mouse_cursor(
    debug_state: Res<DebugState>,
    mut window_q: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = window_q.get_single_mut() {
        // TODO: Make UI egui windows non-interactable and remove the debug_state.enabled check.
        let show_cursor = debug_state.enabled; //&& egui_ctx.ctx_mut().wants_pointer_input();
        window.cursor.visible = show_cursor;
    }
}

fn toggle_debug_ui(
    keys: ResMut<Input<KeyCode>>,
    mut debug_state: ResMut<DebugState>,
    mut egui_ctx: EguiContexts,
) {
    if egui_ctx.ctx_mut().wants_keyboard_input() {
        return;
    }

    if keys.just_pressed(KeyCode::Back) {
        debug_state.enabled = !debug_state.enabled;
    }
}

fn toggle_physics_debug_render(
    keys: ResMut<Input<KeyCode>>,
    mut egui_ctx: EguiContexts,
    mut debug_render_context: ResMut<DebugRenderContext>,
) {
    if egui_ctx.ctx_mut().wants_keyboard_input() {
        return;
    }

    if keys.just_pressed(KeyCode::Key0) {
        debug_render_context.enabled = !debug_render_context.enabled;
    }
}
