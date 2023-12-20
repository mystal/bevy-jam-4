use bevy::prelude::*;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy_egui::EguiContexts;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                camera_control,
            ));
    }
}

pub fn spawn_camera(
    commands: &mut Commands,
    scale: f32,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            projection: OrthographicProjection {
                far: 1000.,
                near: -1000.,
                scale,
                ..default()
            },
            ..default()
        },
        BloomSettings::default(),
    ));
}

fn camera_control(
    mut egui_ctx: EguiContexts,
    mut wheel_events: EventReader<MouseWheel>,
    mut camera_q: Query<(&mut OrthographicProjection, &mut Transform)>,
) {
    if egui_ctx.ctx_mut().wants_pointer_input() {
        return;
    }

    let Ok((mut projection, mut transform)) = camera_q.get_single_mut() else {
        return;
    };

    for event in wheel_events.read() {
        // debug!("Mouse wheel event: {:?}", event);
        match event.unit {
            MouseScrollUnit::Line => {
                projection.scale += 0.1 * -event.y;
                projection.scale = projection.scale.max(0.1);
            },
            MouseScrollUnit::Pixel => todo!(),
        }
    }

    // TODO: Debug keys to move camera around.
}
