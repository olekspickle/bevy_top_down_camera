use bevy::prelude::*;
use bevy_top_down_camera::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TopDownCameraPlugin))
        .add_systems(
            Startup,
            (spawn_camera, spawn_ui, spawn_world, spawn_player).chain(),
        )
        .add_systems(Update, actions)
        .run();
}

#[derive(Component)]
struct Player;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player,
        TopDownCameraTarget, // ADD THIS
        Mesh3d(meshes.add(Capsule3d::new(0.5, 1.0))),
        MeshMaterial3d(materials.add(Color::srgba(0.9, 0.9, 0.9, 0.5))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        IsDefaultUiCamera,
        Camera::default(),
        Camera3d::default(),
        // these are default values, but you can change anything you'd want
        TopDownCamera {
            motion: Motion {
                move_speed: 0.03,
                max_speed: 200.0,
                rotate_speed: 0.01,
                edge_margin: Vec2::splat(30.0),
                ..default()
            },
            zoom: Some((5.0, 50.0).into()),
            height: Some((5.0, 50.0).into()),
            height_rise_key: KeyCode::KeyX.into(),
            height_lower_key: KeyCode::KeyZ.into(),
            rotate_key: MouseButton::Right.into(),
            ..default()
        },
    ));
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.11, 0.27, 0.16))),
    ));

    commands.spawn((
        PointLight {
            intensity: 1500.0 * 1000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0),
    ));
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(50.0),
            height: Val::Percent(22.0),
            align_items: AlignItems::Start,
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![
            (
                Node::default(),
                Text("Move mouse to edges to move camera".to_string()),
            ),
            (Node::default(), Text("Use mouse wheel to zoom".to_string())),
            (Node::default(), Text("X - camera up".to_string())),
            (Node::default(), Text("Z - camera down".to_string())),
            (
                Node::default(),
                Text("F - toggle follow player mode".to_string())
            ),
            (
                Node::default(),
                Text("Right mouse - hold to rotate camera horizontally".to_string())
            ),
        ],
    ));
}

fn actions(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cam_q: Query<&mut TopDownCamera>,
    mut player_q: Query<&mut Transform, With<Player>>,
) {
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if keyboard_input.just_pressed(KeyCode::KeyF) {
        let Ok(mut cam) = cam_q.single_mut() else {
            return;
        };
        cam.motion.follow = !cam.motion.follow;
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
        let speed = 10.0;
        for mut transform in &mut player_q {
            transform.translation += direction * speed * time.delta_secs();
        }
    }
}
