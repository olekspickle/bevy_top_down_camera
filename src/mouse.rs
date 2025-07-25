use super::*;
use bevy::{
    input::{
        ButtonState,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    window::PrimaryWindow,
};

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, move_on_edges)
            .add_systems(Update, (mode_switch, zoom.run_if(zoom_condition)));
    }
}

/// Moves the camera using cursor drag on edges
pub fn move_on_edges(
    time: Res<Time>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut mouse_evr: EventReader<MouseMotion>,
    mut cam_q: Query<(&TopDownCamera, &mut Transform)>,
) {
    let Ok((cam, mut pos)) = cam_q.single_mut() else {
        return;
    };
    let Ok(window) = window_q.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let mut value = Vec2::ZERO;
    for ev in mouse_evr.read() {
        value += ev.delta;
    }

    let mut movement = Vec3::ZERO;
    let mut min_edge_ratio: f32 = 1.0; // Track how close to the edge we are for speed interpolation

    match cam.mode {
        CameraMode::Move => {
            // Horizontal
            {
                let mut dir = *pos.left();
                dir.y = 0.0;
                let dir = dir.normalize_or_zero();

                if cursor_pos.x <= cam.cursor_edge_margin.x {
                    // left edge → move camera left
                    let ratio = cursor_pos.x / cam.cursor_edge_margin.x;
                    min_edge_ratio = min_edge_ratio.min(ratio);
                    movement += dir;
                } else if cursor_pos.x >= window.width() - cam.cursor_edge_margin.x {
                    // right edge → move right
                    let right_offset = window.width() - cursor_pos.x;
                    let ratio = right_offset / cam.cursor_edge_margin.x;
                    min_edge_ratio = min_edge_ratio.min(ratio);
                    movement -= dir;
                }
            }

            // Vertical
            {
                let mut dir = *pos.forward();
                dir.y = 0.0;
                let dir = dir.normalize_or_zero();

                if cursor_pos.y <= cam.cursor_edge_margin.y {
                    let ratio = cursor_pos.y / cam.cursor_edge_margin.y;
                    min_edge_ratio = min_edge_ratio.min(ratio);
                    movement += dir;
                } else if cursor_pos.y >= window.height() - cam.cursor_edge_margin.y {
                    let bottom_offset = window.height() - cursor_pos.y;
                    let ratio = bottom_offset / cam.cursor_edge_margin.y;
                    min_edge_ratio = min_edge_ratio.min(ratio);
                    movement -= dir;
                }
            }

            // Apply movement with adjusted speed
            if movement != Vec3::ZERO {
                let edge_speed_factor = min_edge_ratio.clamp(0.1, 1.0);
                let speed = cam.max_speed * edge_speed_factor;
                pos.translation += movement.normalize_or_zero() * speed * time.delta_secs();
            }
        }
        CameraMode::Rotate => {
            let yaw_rot = Quat::from_rotation_y(-value.x * cam.rotate_speed);
            pos.rotate(yaw_rot);
        }
    }
}

fn zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut cam_q: Query<(&TopDownCamera, &mut Transform)>,
) {
    let Ok((cam, mut pos)) = cam_q.single_mut() else {
        return;
    };

    let mut scroll = 0.0;
    for ev in scroll_evr.read() {
        scroll += ev.y;
    }

    let direction = pos.forward().normalize();
    pos.translation += direction * scroll * cam.zoom.speed;
    // pos.translation.y = pos.translation.y.max(cam.height.max);
}

fn mode_switch(mut events: EventReader<MouseButtonInput>, mut cam_q: Query<&mut TopDownCamera>) {
    let Ok(mut cam) = cam_q.single_mut() else {
        return;
    };

    for ev in events.read() {
        if ev.button == MouseButton::Right {
            match ev.state {
                ButtonState::Pressed => {
                    cam.mode = CameraMode::Rotate;
                }
                ButtonState::Released => {
                    cam.mode = CameraMode::Move;
                }
            }
        }
    }
}

pub fn zoom_condition(cam_q: Query<&TopDownCamera>) -> bool {
    let Ok(cam) = cam_q.single() else {
        return false;
    };
    cam.zoom_enabled
}
