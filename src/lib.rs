use bevy::prelude::*;

mod mouse;
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
        app.add_plugins(MousePlugin).add_systems(
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
    /// Whether camera should follow [`TopDownCameraTarget`] or not
    pub follow: bool,
    pub cursor_enabled: bool,
    /// Value that will be used to lerp camera move speed
    pub cursor_move_speed: f32,
    /// Only relevant if cursor_enabled
    ///
    /// Distance from the edges of the screen in pixels
    /// When cursor enters this edge - camera will start to move with the speed interpolated
    /// between zero and max_speed depending how far into edge you move cursor
    pub cursor_edge_margin: Vec2,
    /// Speed of the rotate action
    pub cursor_rotate_speed: f32,
    pub zoom_enabled: bool,
    /// Only relevant if zoom_enabled
    /// Zoom in/out the map
    pub zoom: Zoom,
    /// Height range of the camera
    pub height: Height,
    pub height_keys_enabled: bool,
    /// Key to lower the camera vertically
    pub height_lower_key: InputType,
    /// Key to rise the camera vertically
    pub height_rise_key: InputType,
    /// Key to rotate the camera horizontally
    pub rotate_key: InputType,
    /// Max speed which will be used in egde interpolation
    pub cursor_max_speed: f32,
    #[doc(hidden)]
    pub mode: CameraMode,
    #[doc(hidden)]
    pub initial_setup: bool,
}

impl TopDownCamera {
    pub fn with_follow(mut self) -> Self {
        self.follow = true;
        self
    }
    pub fn with_cursor(mut self) -> Self {
        self.cursor_enabled = true;
        self
    }
    pub fn with_zoom(mut self) -> Self {
        self.zoom_enabled = true;
        self
    }
    pub fn with_height(mut self) -> Self {
        self.height_keys_enabled = true;
        self
    }

    pub fn with_cursor_move_speed(mut self, speed: f32) -> Self {
        self.cursor_move_speed = speed;
        self
    }

    pub fn with_cursor_edge_margin(mut self, margin: Vec2) -> Self {
        self.cursor_edge_margin = margin;
        self
    }

    pub fn with_cursor_rotate_speed(mut self, speed: f32) -> Self {
        self.cursor_rotate_speed = speed;
        self
    }

    pub fn with_cursor_max_speed(mut self, speed: f32) -> Self {
        self.cursor_max_speed = speed;
        self
    }
    pub fn with_zoom_speed(mut self, speed: f32) -> Self {
        self.zoom.speed = speed;
        self
    }
    pub fn with_zoom_min(mut self, min: f32) -> Self {
        self.zoom.min = min;
        self
    }
    pub fn with_zoom_max(mut self, max: f32) -> Self {
        self.zoom.max = max;
        self
    }
    pub fn with_height_min(mut self, min: f32) -> Self {
        self.height.min = min;
        self
    }
    pub fn with_height_max(mut self, max: f32) -> Self {
        self.height.max = max;
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
    pub fn with_height_keys_enabled(mut self) -> Self {
        self.height_keys_enabled = true;
        self
    }
}

impl Default for TopDownCamera {
    fn default() -> Self {
        TopDownCamera {
            follow: false,
            zoom_enabled: true,
            zoom: (5.0, 50.0).into(),
            cursor_enabled: true,
            cursor_move_speed: 0.2,
            cursor_max_speed: 200.0,
            cursor_rotate_speed: 0.01,
            cursor_edge_margin: Vec2::splat(30.0),
            mode: CameraMode::Move,
            initial_setup: false,
            height: Height::new(5.0, 50.0),
            height_keys_enabled: true,
            height_rise_key: KeyCode::KeyX.into(),
            height_lower_key: KeyCode::KeyZ.into(),
            rotate_key: MouseButton::Right.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputType {
    Key(KeyCode),
    Mouse(MouseButton),
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

    if cam.initial_setup && !cam.follow {
        return;
    }

    for player in player_q.iter() {
        let mut new = player.looking_at(Vec3::new(0.0, 0.0, -10_000.0), Vec3::Y);
        new.rotate_x(-0.65);
        let offset = cam.height.max / 3.0;

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
