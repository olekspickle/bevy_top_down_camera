use bevy::prelude::*;
use bevy_unified_input::*;

mod gamepad;
mod mouse;

pub use gamepad::GamepadPlugin;
pub use mouse::MousePlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CameraSyncSet;

/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_top_down_camera::TopDownCameraPlugin;
/// App::new().add_plugins(TopDownCameraPlugin);
/// ```
pub struct TopDownCameraPlugin;

impl Plugin for TopDownCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MousePlugin, GamepadPlugin));

        app.add_systems(
            PostUpdate,
            (sync_player_camera, change_height.run_if(sync_condition))
                .before(TransformSystems::Propagate)
                .in_set(CameraSyncSet),
        );
    }
}

/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_top_down_camera::TopDownCamera;
/// fn spawn_camera(mut commands: Commands) {
///     commands.spawn((
///         TopDownCamera::default(),
///         Camera::default(),
///         Camera3d::default(),
///     ));
/// }
/// ```
#[derive(Component)]
pub struct TopDownCamera {
    pub motion: Motion,
    pub cursor_enabled: bool,
    pub zoom: Option<Zoom>,
    /// Height range of the camera
    /// If `None` the camera will not move vertically
    pub height: Option<Height>,
    /// Key to lower the camera vertically
    pub height_lower_key: InputBinding,
    /// Key to rise the camera vertically
    pub height_rise_key: InputBinding,
    /// Key to rotate the camera horizontally for mouse and keyboard input
    pub rotate_key: InputBinding,
    /// Gamepad specific settings
    pub gamepad: Option<GamepadInput>,
    #[doc(hidden)]
    pub mode: CameraMode,
    #[doc(hidden)]
    pub initial_setup: bool,
}

impl Default for TopDownCamera {
    fn default() -> Self {
        TopDownCamera {
            motion: Motion::default(),
            cursor_enabled: true,
            mode: CameraMode::Move,
            initial_setup: false,
            zoom: Some((5.0, 50.0).into()),
            height: Some((5.0, 50.0).into()),
            height_rise_key: [KeyCode::KeyX.into(), GamepadButton::DPadUp.into()].into(),
            height_lower_key: [KeyCode::KeyZ.into(), GamepadButton::DPadDown.into()].into(),
            rotate_key: MouseButton::Right.into(),
            gamepad: Some(GamepadInput::default()),
        }
    }
}

pub struct Motion {
    /// Whether camera should follow [`TopDownCameraTarget`] or not
    pub follow: bool,
    /// Value that will be used to lerp camera move speed
    pub move_speed: f32,
    /// Max speed which will be used in egde interpolation
    pub max_speed: f32,
    /// Speed of the rotate action
    pub rotate_speed: f32,
    /// Distance from the edges of the screen in pixels
    /// When cursor enters this edge - camera will start to move with the speed interpolated
    /// between zero and max_speed depending how far into edge you move cursor
    pub edge_margin: Vec2,
    /// Deadzone for gamepad analog sticks
    pub deadzone: f32,
}

impl Default for Motion {
    fn default() -> Self {
        Self {
            follow: true,
            move_speed: 0.05,
            max_speed: 200.0,
            rotate_speed: 0.01,
            edge_margin: Vec2::splat(30.0),
            deadzone: 0.1,
        }
    }
}

pub struct GamepadInput {
    /// Key to zoom in
    pub zoom_in_key: InputBinding,
    /// Key to zoom out
    pub zoom_out_key: InputBinding,
}

impl Default for GamepadInput {
    fn default() -> Self {
        Self {
            zoom_in_key: GamepadButton::RightTrigger2.into(),
            zoom_out_key: GamepadButton::LeftTrigger2.into(),
        }
    }
}

#[derive(Default)]
pub struct Zoom {
    pub min: f32,
    pub max: f32,
    /// Value that will be used to lerp camera zoom speed
    pub speed: f32,
}

impl Zoom {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
            speed: 0.3,
        }
    }
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
}

impl From<(f32, f32)> for Zoom {
    fn from((min, max): (f32, f32)) -> Self {
        Self {
            min,
            max,
            speed: 0.3,
        }
    }
}

#[derive(Default)]
pub struct Height {
    pub min: f32,
    pub max: f32,
}

impl Height {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }
}

impl From<(f32, f32)> for Height {
    fn from((min, max): (f32, f32)) -> Self {
        Self { min, max }
    }
}

#[doc(hidden)]
#[derive(Component, Default, Clone, PartialEq)]
pub enum CameraMode {
    #[default]
    Move,
    Rotate,
}

/// The desired target for the top down camera to look at
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_top_down_camera::TopDownCameraTarget;
/// fn spawn_player(mut commands: Commands) {
///     commands.spawn((
///         Player,
///         TopDownCameraTarget
///     ));
/// }
/// ```
#[derive(Component, Default)]
pub struct TopDownCameraTarget;

fn sync_player_camera(
    target_q: Query<&Transform, With<TopDownCameraTarget>>,
    mut cam_q: Query<(&TopDownCamera, &mut Transform), Without<TopDownCameraTarget>>,
) {
    let Ok((cam, mut pos)) = cam_q.single_mut() else {
        return;
    };

    if !cam.motion.follow {
        return;
    }

    for target in target_q.iter() {
        let mut new = target.looking_at(Vec3::new(0.0, 0.0, -10_000.0), Vec3::Y);
        new.rotate_x(-0.65);
        let offset = cam.height.as_ref().map_or(0.0, |height| height.max / 3.0);

        pos.rotation = new.rotation;
        pos.translation = pos.translation.lerp(
            new.translation + Vec3::new(0.0, offset, offset),
            cam.motion.move_speed,
        );
    }
}

fn change_height(
    keys: Option<Res<ButtonInput<KeyCode>>>,
    gamepad: Option<Res<ButtonInput<GamepadButton>>>,
    mut cam_q: Query<(&TopDownCamera, &mut Transform)>,
) {
    let Ok((cam, mut pos)) = cam_q.single_mut() else {
        return;
    };
    if let Some(height) = cam.height.as_ref() {
        let (rise, lower) = (
            cam.height_rise_key
                .pressed_any(keys.as_ref(), None, gamepad.as_ref()),
            cam.height_lower_key
                .pressed_any(keys.as_ref(), None, gamepad.as_ref()),
        );

        let mut delta = 0.0;
        if rise {
            delta += 1.0;
        }
        if lower {
            delta -= 1.0;
        }

        let target = pos.translation.y + delta;
        if target < height.min || target > height.max {
            return;
        }
        let speed = if let Some(zoom) = cam.zoom.as_ref() {
            zoom.speed
        } else {
            0.1
        };
        pos.translation.y = pos.translation.y.lerp(target, speed);
    }
}

pub fn sync_condition(cam: Single<&TopDownCamera>) -> bool {
    cam.motion.follow
}
