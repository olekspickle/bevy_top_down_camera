use crate::{InputType, TopDownCamera};
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
    mut cam_q: Query<(&mut TopDownCamera, &mut Transform)>,
    gamepad: Query<&Gamepad>,
    axes: If<Res<Axis<GamepadAxis>>>,
    buttons: Res<ButtonInput<GamepadButton>>,
) {
    let Ok((cam, mut pos)) = cam_q.single_mut() else {
        return;
    };

    for _gamepad in gamepad.iter() {
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

            if right_stick_x.abs() > cam.motion.deadzone {
                rotation = -right_stick_x * cam.motion.rotate_speed;
            }

            if movement != Vec3::ZERO {
                let delta = movement * cam.motion.max_speed * time.delta_secs();
                let target = pos.translation + delta;
                pos.translation = pos.translation.lerp(target, cam.motion.move_speed);
            }

            if rotation != 0.0 {
                let yaw_rot = Quat::from_rotation_y(rotation);
                pos.rotate(yaw_rot);
            }

            // Zoom and height
            if let Some(height) = &cam.height {
                let rise_key = gamepad_input.height_rise_key;
                let lower_key = gamepad_input.height_lower_key;

                let mut delta = 0.0;
                if is_gamepad_button_pressed(&buttons, rise_key) {
                    delta += 1.0;
                }
                if is_gamepad_button_pressed(&buttons, lower_key) {
                    delta -= 1.0;
                }

                let target = pos.translation.y + delta;
                if target >= height.min && target <= height.max {
                    let speed = if let Some(zoom) = cam.zoom.as_ref() {
                        zoom.speed
                    } else {
                        0.1
                    };
                    pos.translation.y = pos.translation.y.lerp(target, speed);
                }
            }

            if let Some(zoom) = &cam.zoom {
                let zoom_in_key = gamepad_input.zoom_in_key;
                let zoom_out_key = gamepad_input.zoom_out_key;

                let mut scroll = 0.0;
                if is_gamepad_button_pressed(&buttons, zoom_in_key) {
                    scroll += 1.0;
                }
                if is_gamepad_button_pressed(&buttons, zoom_out_key) {
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

fn is_gamepad_button_pressed(buttons: &Res<ButtonInput<GamepadButton>>, input: InputType) -> bool {
    if let InputType::Gamepad(button_type) = input {
        buttons.pressed(button_type)
    } else {
        false
    }
}
