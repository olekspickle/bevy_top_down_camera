use bevy::prelude::*;

mod gamepad;
mod mouse;
use gamepad::GamepadPlugin;
use mouse::MousePlugin;

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
            sync_player_camera
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
///         Camera3dBundle::default()
///     ));
/// }
/// ```
#[derive(Component)]
pub struct TopDownCamera {
    pub motion: Motion,
    pub cursor_enabled: bool,
    pub zoom: Option<Zoom>,
    /// Height range of the camera
    pub height: Option<Height>,
    /// Key to lower the camera vertically
    pub height_lower_key: InputType,
    /// Key to rise the camera vertically
    pub height_rise_key: InputType,
    /// Key to rotate the camera horizontally
    pub rotate_key: InputType,

    #[doc(hidden)]
    pub mode: CameraMode,
    #[doc(hidden)]
    pub initial_setup: bool,
    pub gamepad: Option<GamepadInput>,
}

pub struct GamepadInput {
    /// Key to zoom in
    pub zoom_in_key: InputType,
    /// Key to zoom out
    pub zoom_out_key: InputType,
    /// Key to rise the camera vertically
    pub height_rise_key: InputType,
    /// Key to lower the camera vertically
    pub height_lower_key: InputType,
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

impl TopDownCamera {
    pub fn with_follow(mut self, follow: bool) -> Self {
        self.motion.follow = follow;
        self
    }

    pub fn with_move_speed(mut self, speed: f32) -> Self {
        self.motion.move_speed = speed;
        self
    }

    pub fn with_max_speed(mut self, speed: f32) -> Self {
        self.motion.max_speed = speed;
        self
    }

    pub fn with_rotate_speed(mut self, speed: f32) -> Self {
        self.motion.rotate_speed = speed;
        self
    }

    pub fn with_edge_margin(mut self, margin: Vec2) -> Self {
        self.motion.edge_margin = margin;
        self
    }

    pub fn with_zoom_speed(mut self, speed: f32) -> Self {
        self.zoom.get_or_insert_with(Default::default).speed = speed;
        self
    }

    pub fn with_zoom_min(mut self, min: f32) -> Self {
        self.zoom.get_or_insert_with(Default::default).min = min;
        self
    }

    pub fn with_zoom_max(mut self, max: f32) -> Self {
        self.zoom.get_or_insert_with(Default::default).max = max;
        self
    }

    pub fn with_height_min(mut self, min: f32) -> Self {
        self.height.get_or_insert_with(Default::default).min = min;
        self
    }

    pub fn with_height_max(mut self, max: f32) -> Self {
        self.height.get_or_insert_with(Default::default).max = max;
        self
    }

    pub fn with_height_rise_key(mut self, key: KeyCode) -> Self {
        self.height_rise_key = key.into();
        self
    }
    pub fn with_height_lower_key(mut self, key: KeyCode) -> Self {
        self.height_lower_key = key.into();
        self
    }
    pub fn with_rotate_key(mut self, key: KeyCode) -> Self {
        self.rotate_key = key.into();
        self
    }
}

impl Default for TopDownCamera {
    fn default() -> Self {
        TopDownCamera {
            motion: Motion::default(),
            cursor_enabled: true,
            zoom: Some((5.0, 50.0).into()),
            mode: CameraMode::Move,
            initial_setup: false,
            height: Some(Height::new(5.0, 50.0)),
            height_rise_key: KeyCode::KeyX.into(),
            height_lower_key: KeyCode::KeyZ.into(),
            rotate_key: MouseButton::Right.into(),
            gamepad: Some(GamepadInput::default()),
        }
    }
}

impl Default for GamepadInput {
    fn default() -> Self {
        Self {
            zoom_in_key: GamepadButton::RightTrigger2.into(),
            zoom_out_key: GamepadButton::LeftTrigger2.into(),
            height_rise_key: GamepadButton::DPadUp.into(),
            height_lower_key: GamepadButton::DPadDown.into(),
        }
    }
}

impl Default for Motion {
    fn default() -> Self {
        Self {
            follow: false,
            move_speed: 0.2,
            max_speed: 200.0,
            rotate_speed: 0.01,
            edge_margin: Vec2::splat(30.0),
            deadzone: 0.1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputType {
    Key(KeyCode),
    Mouse(MouseButton),
    Gamepad(GamepadButton),
}
impl InputType {
    fn key(&self) -> KeyCode {
        match self {
            InputType::Key(key) => *key,
            _ => unreachable!("not key"),
        }
    }
}
impl From<KeyCode> for InputType {
    fn from(value: KeyCode) -> Self {
        InputType::Key(value)
    }
}
impl From<MouseButton> for InputType {
    fn from(value: MouseButton) -> Self {
        InputType::Mouse(value)
    }
}

impl From<GamepadButton> for InputType {
    fn from(value: GamepadButton) -> Self {
        InputType::Gamepad(value)
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

/// The desired target for the top down camera to look at
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_top_down_camera::TopDownCameraTarget;
/// fn spawn_player(mut commands: Commands) {
///     commands.spawn((
///         PbrBundle::default(),
///         TopDownCameraTarget
///     ));
/// }
/// ```
#[derive(Component)]
pub struct TopDownCameraTarget;

fn sync_player_camera(
    player_q: Query<&Transform, With<TopDownCameraTarget>>,
    mut cam_q: Query<(&mut TopDownCamera, &mut Transform), Without<TopDownCameraTarget>>,
) {
    let Ok((mut cam, mut pos)) = cam_q.single_mut() else {
        return;
    };

    if cam.initial_setup && !cam.motion.follow {
        return;
    }

    for player in player_q.iter() {
        let mut new = player.looking_at(Vec3::new(0.0, 0.0, -10_000.0), Vec3::Y);
        new.rotate_x(-0.65);
        let offset = if let Some(height) = cam.height.as_ref() {
            height.max / 3.0
        } else {
            0.0
        };

        pos.rotation = new.rotation;
        pos.translation = new.translation + Vec3::new(0.0, offset, offset);
    }

    if !cam.initial_setup {
        cam.initial_setup = true;
    }
}

#[doc(hidden)]
#[derive(Component, Default, Clone, PartialEq)]
pub enum CameraMode {
    #[default]
    Move,
    Rotate,
}
