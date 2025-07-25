use bevy::prelude::*;
use bevy_top_down_camera::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TopDownCameraPlugin /* ADD THIS */))
        .add_systems(Startup, (spawn_player, spawn_world, spawn_camera))
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
    let mesh = Mesh::from(Capsule3d::new(0.5, 1.0));
    let mesh = Mesh3d(meshes.add(mesh.clone()));
    let color: MeshMaterial3d<StandardMaterial> =
        MeshMaterial3d(materials.add(Color::srgba(0.9, 0.9, 0.9, 0.5)));

    let player = (
        mesh,
        color,
        Player,
        TopDownCameraTarget, // ADD THIS
        Transform::from_xyz(0.0, 0.5, 0.0),
    );

    commands.spawn(player);
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3d::default(),
        TopDownCamera::default(), // ADD THIS
    );
    commands.spawn(camera);
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        Mesh3d(meshes.add(Mesh::from(Plane3d::default().mesh().size(50.0, 50.0)))),
        MeshMaterial3d(materials.add(Color::srgb(0.11, 0.27, 0.16))),
    );

    let light = (
        PointLight {
            intensity: 1500.0 * 1000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0),
    );

    commands.spawn(floor);
    commands.spawn(light);

    commands.spawn((
        Node {
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Node::default(),
                Text("Move mouse to edges to move camera".to_string())
            ),
            (Node::default(), Text("Use mouse wheel to zoom".to_string())),
            (Node::default(), Text("A - camera up".to_string())),
            (Node::default(), Text("Z - camera down".to_string())),
            (
                Node::default(),
                Text("Right mouse - hold to rotate camera horizontally".to_string()),
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

    if keyboard_input.pressed(KeyCode::KeyF) {
        let Ok(mut cam) = cam_q.single_mut() else {
            return;
        };
        cam.follow = !cam.follow;
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
        let speed = 10.0;
        for mut transform in &mut player_q {
            transform.translation += direction * speed * time.delta_secs();
        }
    }
}
