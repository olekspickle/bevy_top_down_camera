use crate::TopDownCamera;
use bevy::{
    input::gamepad::{GamepadConnection, GamepadConnectionEvent},
    prelude::*,
};

pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, gamepad_input)
            .add_systems(Update, gamepad_connection_events);
    }
}

fn gamepad_connection_events(mut events: MessageReader<GamepadConnectionEvent>) {
    for event in events.read() {
        match &event.connection {
            GamepadConnection::Connected {
                name,
                vendor_id,
                product_id,
            } => {
                tracing::info!(
                    "Gamepad connected: {} {name}{}{}",
                    event.gamepad,
                    vendor_id.map(|s| format!("{s},")).unwrap_or_default(),
                    product_id.map(|s| format!("{s},")).unwrap_or_default()
                );
            }
            GamepadConnection::Disconnected => {
                tracing::info!("Gamepad connected");
            }
        }
    }
}

fn gamepad_input(
    time: Res<Time>,
    axes: If<Res<Axis<GamepadAxis>>>,
    // TODO: zoom in\out on gamepad touchpad?..
    // mut scroll_gamepad_evr: MessageReader<GamepadAxis>,
    gamepads: Query<&Gamepad>,
    gamepad_btn: Res<ButtonInput<GamepadButton>>,
    mut cam_q: Query<(&mut TopDownCamera, &mut Transform)>,
) {
    let Ok((cam, mut pos)) = cam_q.single_mut() else {
        return;
    };

    for _ in gamepads.iter() {
        if let Some(gamepad_input) = &cam.gamepad {
            // Movement and rotation
            let left_stick_x = axes.get(GamepadAxis::LeftStickX).unwrap_or_default();
            let left_stick_y = axes.get(GamepadAxis::LeftStickY).unwrap_or_default();
            let right_stick_x = axes.get(GamepadAxis::RightStickX).unwrap_or_default();

            let mut movement = Vec3::ZERO;
            let mut rotation = 0.0;

            if left_stick_x.abs() > cam.motion.deadzone {
                let mut dir = *pos.left();
                dir.y = 0.0;
                let dir = dir.normalize_or_zero();
                movement += dir * -left_stick_x;
            }
            if left_stick_y.abs() > cam.motion.deadzone {
                let mut dir = *pos.forward();
                dir.y = 0.0;
                let dir = dir.normalize_or_zero();
                movement += dir * left_stick_y;
            }
            if movement != Vec3::ZERO {
                let delta = movement * cam.motion.max_speed * time.delta_secs();
                let target = pos.translation + delta;
                pos.translation = pos.translation.lerp(target, cam.motion.move_speed);
            }

            if right_stick_x.abs() > cam.motion.deadzone {
                rotation = -right_stick_x * cam.motion.rotate_speed;
            }
            if rotation != 0.0 {
                let yaw_rot = Quat::from_rotation_y(rotation);
                pos.rotate(yaw_rot);
            }

            // Zoom
            if let Some(zoom) = &cam.zoom {
                let mut scroll = 0.0;
                if gamepad_input.zoom_in_key.pressed_gamepad(&gamepad_btn) {
                    scroll += 1.0;
                }
                if gamepad_input.zoom_out_key.pressed_gamepad(&gamepad_btn) {
                    scroll -= 1.0;
                }
                if scroll != 0.0 {
                    let direction = pos.forward().normalize();
                    let delta = direction * scroll;
                    let target = pos.translation + delta;
                    if let Some(height) = cam.height.as_ref()
                        && target.y >= height.min
                        && target.y <= height.max
                    {
                        pos.translation = pos.translation.lerp(target, zoom.speed);
                    }
                }
            }
        }
    }
}
