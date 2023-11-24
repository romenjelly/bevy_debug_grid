# Bevy Debug Grid

[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

A plugin for creating debug mesh grids in the [bevy](https://bevyengine.org/) game engine.

![default_cube](./assets/default_cube.png "the default cube example")
*The `default_cube` example*

## Installation

To install this plugin, add the following to your `Cargo.toml`:

```toml
[dependencies]
bevy_debug_grid = "0.4"
```

## Setup

To use the plugin, import it: `use bevy_debug_grid::*;`  
Then add the provided `DebugGridPlugin` plugin to your app.

```rs
use bevy::prelude::*;
use bevy_debug_grid::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DebugGridPlugin::with_floor_grid(),
        ))
        .run();
}
```

It is also possible to avoid spawning a default floor grid by doing `.add_plugins(DebugGridPlugin::without_floor_grid())`

## Examples

Several examples are provided, they can be launched by cloning this repository and running `cargo run --example <example name>`

All examples use the [`bevy_spectator` camera plugin](https://github.com/JonahPlusPlus/bevy_spectator) for movement. Use the `W` `A` `S` `D`, `Shift`, `Space`, and `CTRL` keys to move. Use `Esc` to release the cursor.  
The camera spawns at world origin, so it needs to be moved a bit to see the examples in action.

Here's an exhaustive list of the provided examples:

- `default_cube` - the minimal example, resembling Blender's default scene. Has a tracked grid and a cube at world origin (+0.5_f32 on Y).
- `moving_grid` - an example of how a grid can be transformed, either by moving it, or by moving its parent.
- `changing_grid` - an example of dynamically changing the properties of grids and sub-grids
- `changing_grid_axis` - an example of dynamically changing grid axis appearance
- `dynamic_floor_grid` - an example of a custom floor grid tracked on the X axis and a dynamic offset

## Behavior

All grids spawned by this plugin are meshes, rendered with a `polygonMode` of `PolygonMode::Line`.  
An alternative would've been to create a plane and draw the lines using a shader, but it's not what has been chosen for this plugin.

This plugin's components work by spawning marked children. For example, a `Grid` will spawn a `GridChild` which will contain a `Mesh` and a `Material`.  
This has implications regarding transforming the grid.

- If there is no need to transform the grid separately, it can be spawned on the same level as all other components of the entity
- If the grid needs to be transformed relative to its parent, spawn it as a child of the the entity

An demonstration of this can seen by running the `moving_grid` example.

## Features

### Grid

The `Grid` component spawns a configurable a mesh grid.

```rs
commands.spawn((
    Grid {
        // Space between each line
        spacing: 1.0_f32,
        // Line count along a single axis
        count: 8,
        // Color of the lines
        color: Color::SILVER,
        // Alpha mode for all components
        alpha_mode: AlphaMode::Opaque,
    },
    TransformBundle::default(),
    VisibilityBundle::default(),
));
```

The `Grid::default()` is a small silver grid with 8 lines per axis and a spacing of `0.25_f32` between them.

Grids have an `alpha_mode`, which determines the alpha mode for the grid material, as well as all other related materials, such as sub-grids, and grid axis.  
The `color` should have an alpha value for alpha modes outside of `AlphaMode::Opaque` to have a visible effect.  
The default alpha mode for grids is `AlphaMode::Blend`.

### Sub-Grid

The `SubGrid` component spawns a configurable mesh sub-grid when added next to a grid.  
It creates lines between the lines of the main grid.

```rs
commands.spawn((
    Grid { /* ... */ },
    SubGrid {
        // Line count between each line of the main grid
        count: 4,
        // Line color
        color: Color::GRAY,
    },
    // Other components...
));
```

### Grid Axis Color Overrides

The `GridAxis` component allows for setting custom colors per grid axis.

```rs
commands.spawn((
    Grid { /* ... */ },
    GridAxis {
        x: Some(Color::RED),
        z: Some(Color::Blue),
        // Fills the remaining axis with None
        ..default()
    },
    // Other components...
));
```

### Tracked Grid

The `TrackedGrid` makes a grid tracked along a given axis.  
The grid will move along with the camera and have its material clip at a certain distance, creating the illusion of an infinite grid.

```rs
commands.spawn((
    Grid { /* ... */ },
    TrackedGrid {
        // This will track it as a "floor"
        alignment: GridAlignment::Y,
        // The offset along the Y axis
        offset: 0.0_f32,
    },
    // Other components...
));
```

### Custom Tracking for Grids

Tracked grids have the illusion of being infinite by physically being moved next to the camera and some shader trickery. However, this can be an issue when multiple cameras are involved.

#### Custom Tracking Using Generics

To achieve generic custom tracking, the world must contain exactly one entity with a marker component which is desired to be tracked. The `DebugGridPlugin` tracks a `Camera`, since it is a type alias for `TrackedDebugGridPlugin::<Camera>`.

To track a component other than a `Camera`, add the `TrackedDebugGridPlugin::<T>` instead of the `DebugGridPlugin` to your app, where `T` is the component which should be tracked.

Example:

```rs
use bevy::prelude::*;
use bevy_debug_grid::*;

// Custom component to track grids
#[derive(Component)]
struct MainCamera;

// Spawns the main camera
fn spawn_main_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle::default(),
        MainCamera,
    ));
}

// Reads inputs to control the camera
fn control_main_camera(/* ... */) { /* ... */ }

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TrackedDebugGridPlugin::<MainCamera>::with_floor_grid(),
        ))
        .add_systems(Startup, spawn_main_camera)
        .add_systems(Update, control_main_camera)
        .run();
}
```

Tracked grids will now track alongside the entity which has the `MainCamera` component.
In the above example, tracked grids will now track alongside the entity which has the `MainCamera` component.

#### Custom Tracking Using Entity Overrides

Grids can be tracked by setting a custom entity override.

This can be required when rendering to a different camera which renders to a texture, likely on a different render layer.

This override is defined by setting the `tracking_override` of a `TrackedGrid`.

```rs
let entity = commands.spawn(
    // Component bundle...
).id(); // Get the Entity

commands.spawn((
    Grid { /* ... */ },
    TrackedGrid {
        // Set the entity as the tracking override
        tracking_override: Some(entity),
        ..default()
    },
    // Other components...
));
```

A tracked grid with an override will then no longer be tracked to the plugin's generic component, and instead follow the given entity.

If the entity is despawned, the tracked grid will stop updating its position.

### Render Layers

Adding a `RenderLayers` component to an entity with a `Grid` will ensure that all spawned grid meshes will also contain the same `RenderLayers`.

`RenderLayers` do also participate in change detection when updating grid properties.

## Known Bugs & Missing Features

- *Bug:* removing `TrackedGrid` or `GridAxis` will not properly update the other components. It will currently just break. Current workaround is to despawn the entity.

## Compatibility

| Bevy Version | Plugin Version |
|:------------:|:--------------:|
|    `0.12`    |  `0.3.0-0.4.0` |
|    `0.11`    |  `0.2.0-0.2.1` |
|    `0.10`    |  `0.1.0-0.1.1` |

## License

This plugin is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
