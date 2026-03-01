use super::*;
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    window::PrimaryWindow,
};

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (move_on_edges, mode_switch))
            .add_systems(Update, zoom.run_if(zoom_condition));
    }
}

/// Moves the camera using cursor drag on edges
pub fn move_on_edges(
    time: Res<Time>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut mouse_evr: MessageReader<MouseMotion>,
    mut cam_q: Query<(&TopDownCamera, &mut Transform)>,
) {
    let Ok((cam, mut pos)) = cam_q.single_mut() else {
        return;
    };
    if !cam.cursor_enabled || cam.motion.follow {
        return;
    }

    let Ok(window) = window_q.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let value = mouse_evr.read().fold(Vec2::ZERO, |acc, ev| acc + ev.delta);

    match cam.mode {
        CameraMode::Move => {
            let mut movement = Vec3::ZERO;
            let mut edge_rel_speed: f32 = 1.0; // Track how close to the edge we are for speed interpolation

            // Horizontal
            {
                let mut dir = *pos.left();
                dir.y = 0.0;
                let dir = dir.normalize_or_zero();

                if cursor_pos.x <= cam.motion.edge_margin.x {
                    // left edge → move camera left
                    let ratio = cursor_pos.x / cam.motion.edge_margin.x;
                    edge_rel_speed = edge_rel_speed.min(ratio);
                    movement += dir;
                } else if cursor_pos.x >= window.width() - cam.motion.edge_margin.x {
                    // right edge → move right
                    let right_offset = window.width() - cursor_pos.x;
                    let ratio = right_offset / cam.motion.edge_margin.x;
                    edge_rel_speed = edge_rel_speed.min(ratio);
                    movement -= dir;
                }
            }

            // Vertical
            {
                let mut dir = *pos.forward();
                dir.y = 0.0;
                let dir = dir.normalize_or_zero();

                if cursor_pos.y <= cam.motion.edge_margin.y {
                    let ratio = cursor_pos.y / cam.motion.edge_margin.y;
                    edge_rel_speed = edge_rel_speed.min(ratio);
                    movement += dir;
                } else if cursor_pos.y >= window.height() - cam.motion.edge_margin.y {
                    let bottom_offset = window.height() - cursor_pos.y;
                    let ratio = bottom_offset / cam.motion.edge_margin.y;
                    edge_rel_speed = edge_rel_speed.min(ratio);
                    movement -= dir;
                }
            }

            // Apply movement with adjusted speed
            if movement != Vec3::ZERO {
                let edge_rel_speed = edge_rel_speed.max(cam.motion.move_speed);
                let speed = cam.motion.max_speed / edge_rel_speed;
                let delta = movement.normalize_or_zero() * speed * time.delta_secs();
                let target = pos.translation + delta;
                pos.translation = pos.translation.lerp(target, cam.motion.move_speed);
            }
        }
        CameraMode::Rotate => {
            let yaw_rot = Quat::from_rotation_y(-value.x * cam.motion.rotate_speed);
            pos.rotate(yaw_rot);
        }
    }
}

fn zoom(
    mut scroll_evr: MessageReader<MouseWheel>,
    mut cam_q: Query<(&TopDownCamera, &mut Transform)>,
) {
    let Ok((cam, mut pos)) = cam_q.single_mut() else {
        return;
    };
    if let (Some(height), Some(zoom)) = (cam.height.as_ref(), cam.zoom.as_ref()) {
        let mut scroll = 0.0;
        for ev in scroll_evr.read() {
            scroll += ev.y;
        }

        if scroll == 0.0 {
            return;
        }

        let direction = pos.forward().normalize();
        let delta = direction * scroll;
        let target = pos.translation + delta;
        if target.y < height.min || target.y > height.max {
            return;
        }
        pos.translation = pos.translation.lerp(target, zoom.speed);
    }
}

fn zoom_condition(cam: Single<&TopDownCamera>) -> bool {
    cam.zoom.is_some()
}

fn mode_switch(
    keys: Option<Res<ButtonInput<KeyCode>>>,
    mouse: Option<Res<ButtonInput<MouseButton>>>,
    mut cam_q: Query<&mut TopDownCamera>,
) {
    let Ok(mut cam) = cam_q.single_mut() else {
        return;
    };

    let rotate_pressed = cam
        .rotate_key
        .just_pressed_any(keys.as_ref(), mouse.as_ref(), None);

    if rotate_pressed {
        cam.mode = CameraMode::Rotate;
    } else {
        cam.mode = CameraMode::Move;
    }
}
