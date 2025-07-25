# Bevy Top Down Camera

- Move around by hovering cursor to the edges of the screen
- Zoom in/out
- Rotate horizontally to change observable angle preserving pitch and yaw
- Follow target

![camera demo](assets/top-down.gif)

## Getting Started

Add the **bevy_to_down_camera** crate:

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
- custom

```
cargo run --example <example name>
```

## Custom Settings

Most settings can be overridden:

```rust
commands.spawn((
    // These are the default settings
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

| Action             | Mouse/Keyboard      | Gamepad      | Enabled by Default |
| ------------------ | ------------------- | ------------ | ------------------ |
| Zoom In            | Scroll Up           | D Pad Up     | Yes                |
| Zoom Out           | Scroll Down         | D Pad Down   | Yes                |
| Aim                | Right Mouse Button  | Left Trigger | No                 |
| Toggle Offset      | E                   | D Pad Right  | No                 |
| Cursor Lock/Unlock | Space               | n/a          | Yes                |
| Orbit Button       | Middle Mouse Button | Left Bumper  | No                 |

## Bevy Version Compatibility

| bevy | bevy_third_person_camera |
| ---- | ------------------------ |
| 0.16 | 0.2.1 - 0.3              |
| 0.15 | 0.2.0                    |
| 0.14 | 0.1.11 - 0.1.14          |
| 0.13 | 0.1.9 - 0.1.10           |
| 0.12 | 0.1.7 - 0.1.8            |
| 0.11 | 0.1.1 - 0.1.6            |

Refer to the [Changelog](Changelog.md) to view breaking changes and updates.

## Migration Guides

- [v0.1.14 -> v0.2.0](migrationGuides/v0.1.14-v0.2.0.md)
- [v0.1.10 -> v0.1.11](migrationGuides/v0.1.10-v0.1.11.md)
- [v0.1.9 -> v0.1.10](migrationGuides/v0.1.9-v0.1.10.md)

## License

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)





