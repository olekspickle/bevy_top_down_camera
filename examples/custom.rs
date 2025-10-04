use bevy::prelude::*;
use bevy_top_down_camera::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TopDownCameraPlugin))
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
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.5, 1.0)),
            material: materials.add(Color::srgba(0.9, 0.9, 0.9, 0.5)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
        TopDownCameraTarget, // ADD THIS
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        // these are default values, but you can change anything you'd want
        TopDownCamera {
            motion: Motion {
                follow: false,
                move_speed: 0.2,
                max_speed: 200.0,
                rotate_speed: 0.01,
                edge_margin: Vec2::splat(30.0),
                deadzone: 0.1,
            },
            zoom: Some((5.0, 50.0).into()),
            height: Some(Height::new(5.0, 50.0)),
            height_rise_key: KeyCode::KeyX.into(),
            height_lower_key: KeyCode::KeyZ.into(),
            rotate_key: MouseButton::Right.into(),
            ..default()
        }, // ADD THIS
    ));
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::srgb(0.11, 0.27, 0.16)),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0 * 1000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    });

    commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Move mouse to edges to move camera",
                TextStyle::default(),
            ));
            parent.spawn(TextBundle::from_section(
                "Use mouse wheel to zoom",
                TextStyle::default(),
            ));
            parent.spawn(TextBundle::from_section("X - camera up", TextStyle::default()));
            parent.spawn(TextBundle::from_section("Z - camera down", TextStyle::default()));
            parent.spawn(TextBundle::from_section(
                "F - toggle follow player mode",
                TextStyle::default(),
            ));
            parent.spawn(TextBundle::from_section(
                "Right mouse - hold to rotate camera horizontally",
                TextStyle::default(),
            ));
        }),
    );
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
