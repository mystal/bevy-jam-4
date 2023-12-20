use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{
    egui::{self, Align2, Color32, ComboBox, DragValue, Frame, RichText},
    EguiContexts,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::render::{DebugRenderContext, RapierDebugRenderPlugin};
use num_enum::{IntoPrimitive, FromPrimitive};
use strum::{EnumCount, EnumVariantNames, VariantNames};

use crate::{
    // enemies::spawner::Spawner,
    game::{ai, enemies, input, units::{self, SwarmParent}},
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
                debug_menu_bar.run_if(debug_ui_enabled),
                debug_ui,
                toggle_debug_ui,
                toggle_physics_debug_render,
                place_entity.run_if(place_entity_mode_enabled),
            ).before(input::read_player_input))
            .add_systems(Last, update_mouse_cursor);
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, EnumCount, EnumVariantNames, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
enum PlaceEntityMode {
    #[default]
    None,
    Enemy,
}

#[derive(Resource)]
struct DebugState {
    enabled: bool,
    show_world_inspector: bool,
    resize_swarm_count: u32,
    place_entity_mode: PlaceEntityMode,
}

impl Default for DebugState {
    fn default() -> Self {
        Self {
            enabled: false,
            show_world_inspector: false,
            resize_swarm_count: 10,
            place_entity_mode: default(),
        }
    }
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

fn place_entity_mode_enabled(
    debug_ui: Res<DebugState>,
) -> bool {
    debug_ui.enabled && debug_ui.place_entity_mode != PlaceEntityMode::None
}

fn debug_menu_bar(
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
                    // ui.checkbox(&mut debug_state.place_entity_mode, "Place Entity Mode");
                });
            });
        });
}

fn debug_ui(
    mut commands: Commands,
    mut debug_state: ResMut<DebugState>,
    mut egui_ctx: EguiContexts,
    swarm_q: Query<(Entity, &Children), With<SwarmParent>>,
) {
    let ctx = egui_ctx.ctx_mut();
    let swarm_size = swarm_q.get_single()
        .map(|(_, children)| children.len())
        .unwrap_or_default();
    let swarm_text = RichText::new(format!("Swarm Size: {}", swarm_size))
        .color(Color32::WHITE)
        .size(20.0);

    if debug_state.enabled {
        egui::Window::new("temp_side_panel")
            .anchor(Align2::RIGHT_TOP, (-10.0, 100.0))
            .title_bar(false)
            .collapsible(false)
            .auto_sized()
            .show(ctx, |ui| {
                ui.label(swarm_text);

                ui.horizontal(|ui| {
                    if ui.button("Resize Swarm").clicked() {
                        units::resize_swarm(&mut commands, &swarm_q, debug_state.resize_swarm_count)
                    }

                    ui.add(DragValue::new(&mut debug_state.resize_swarm_count));
                });

                let selected: u8 = debug_state.place_entity_mode.into();
                let mut selected = selected as usize;
                ComboBox::from_label("Place Entity")
                    .show_index(ui, &mut selected, PlaceEntityMode::COUNT, |i| PlaceEntityMode::VARIANTS[i]);
                debug_state.place_entity_mode = PlaceEntityMode::from(selected as u8);
            });
    } else {
        egui::Window::new("temp_side_panel")
            .anchor(Align2::RIGHT_TOP, (-10.0, 100.0))
            .title_bar(false)
            .collapsible(false)
            .frame(Frame::none())
            .auto_sized()
            .show(ctx, |ui| {
                ui.label(swarm_text);
            });
    }
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

fn place_entity(
    mut commands: Commands,
    mouse_buttons: Res<Input<MouseButton>>,
    mut egui_ctx: EguiContexts,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    primary_window_q: Query<&Window, With<PrimaryWindow>>,
) {
    if egui_ctx.ctx_mut().wants_pointer_input() || !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(pos) = input::get_mouse_world_pos(&primary_window_q, &camera_q) {
        commands.spawn((
            enemies::EnemyBundle::new(pos),
            ai::SimpleShooterAi::new(3.0, 0.0..=2.0),
        ));
    }
}
