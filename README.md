# Bevy Top Down Camera

- Move around by hovering cursor to the edges of the screen
- Zoom in/out
- Rotate horizontally to change observable angle preserving pitch and yaw
- Follow target

![camera demo](assets/top-down.gif)

## Getting Started

Add the **bevy_top_down_camera** crate:

```
cargo add bevy_third_person_camera
```

Import the **bevy_top_down_camera** crate:

```rust
use bevy_top_down_camera::*;
```

Add the **TopDownCameraPlugin**:

```rust
.add_plugins(TopDownCameraPlugin)
```

Add the **TopDownCamera** component to the camera entity:

```rust
commands.spawn((
    Camera3d::default(),
    TopDownCamera::default(),
));
```
If you want camera to follow the player, add the **TopDownCameraTarget** component to your player:

```rust
// Player
commands.spawn((
    MeshMaterial3d(materials.add(Color::WHITE)),
    Mesh3d(meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)))),
    Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
    TopDownCameraTarget,
    Player,
));
```

## Examples

- default

```
cargo run --example <example name>
```

## Custom Settings

Most settings can be overridden:

```rust
commands.spawn((
    // These are the default settings
    TopDownCamera {
        follow: true,
        zoom_enabled: true,
        zoom: Zoom::new(5.0, 50.0, 10.0),
        cursor_enabled: true,
        cursor_edge_margin: Vec2::splat(30.0),
        height: Height::new(5.0, 50.0),
        rotate_speed: 0.01,
        max_speed: 200.0,
        ..default()
    },
    Camera3d::default(),
));
```

## Physics Support

When using third party physics engines such as bevy rapier 3d or avian 3d, you should force the 'sync_player_camera' system to run *after* the physics systems. Failing to do this will cause a jittering effect to occur when applying forces/impulses to an object that has a camera entity attached. Simply add the following to your App::new() method (also see examples/physics.rs for complete example):

```rust
.configure_sets(PostUpdate, CameraSyncSet.after(PhysicsSet::StepSimulation)) // Bevy Rapier 3d
.configure_sets(PostUpdate, CameraSyncSet.after(PhysicsSet::Sync)) // Avian 3d
```

## Default Controls

| Action             | Mouse/Keyboard        |  Enabled by Default |
| ------------------ | -------------------   |  ------------------ |
| Zoom In            | Scroll Up             |  Yes                |
| Zoom Out           | Scroll Down           |  Yes                |
| Rotate             | Right Mouse Button    |  Yes                |
| Move around        | Hover to screen edges |  Yes                |
| Follow             | -                     |  No                 |

## Bevy Version Compatibility

| bevy | bevy_top_down_camera |
| ---- | ------------------------ |
| 0.16 | 0.1.0 - 0.1.1              |

## License

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

