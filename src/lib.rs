use bevy::prelude::*;

mod mouse;

use mouse::MousePlugin;

/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_top_down_camera::TopDownCameraPlugin;
/// App::new().add_plugins(TopDownCameraPlugin);
/// ```
pub struct TopDownCameraPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CameraSyncSet;

impl Plugin for TopDownCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MousePlugin).add_systems(
            PostUpdate,
            sync_player_camera
                .before(TransformSystem::TransformPropagate)
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
    pub follow: bool,
    pub cursor_enabled: bool,
    /// Only relevant if cursor_enabled
    ///
    /// Distance from the edges of the screen in pixels
    /// When cursor enters this edge - camera will start to move with the speed interpolated
    /// between zero and max_speed depending how far into edge you move cursor
    pub cursor_edge_margin: Vec2,
    pub zoom_enabled: bool,
    /// Only relevant if zoom_enabled
    /// Zoom in/out the map
    pub zoom: Zoom,
    /// Height range of the camera
    pub height: Height,
    pub max_speed: f32,
    /// Speed of the rotate action
    pub rotate_speed: f32,
    #[doc(hidden)]
    pub mode: CameraMode,
    #[doc(hidden)]
    pub initial_setup: bool,
}

impl Default for TopDownCamera {
    fn default() -> Self {
        TopDownCamera {
            initial_setup: false,
            follow: false,
            zoom_enabled: true,
            zoom: Zoom::new(5.0, 50.0, 10.0),
            height: Height::new(5.0, 50.0),
            cursor_enabled: true,
            cursor_edge_margin: Vec2::splat(30.0),
            rotate_speed: 0.01,
            max_speed: 200.0,
            mode: CameraMode::Move,
        }
    }
}

pub struct Zoom {
    pub min: f32,
    pub max: f32,
    pub speed: f32,
}

impl Zoom {
    pub fn new(min: f32, max: f32, speed: f32) -> Self {
        Self { min, max, speed }
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
        let mut new = player.looking_at(Vec3::new(0.0, 0.0, -f32::INFINITY), Vec3::Y);
        new.rotate_x(-0.65);
        let offset = cam.height.max / 2.0;

        pos.rotation = new.rotation;
        pos.translation = new.translation + Vec3::new(0.0, offset, offset);
    }

    if !cam.initial_setup {
        cam.initial_setup = true;
    }
}

#[doc(hidden)]
#[derive(Component, Default)]
pub enum CameraMode {
    #[default]
    Move,
    Rotate,
}
