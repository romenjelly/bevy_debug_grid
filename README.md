# Bevy Debug Grid

[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

A plugin for creating debug mesh grids in the [bevy](https://bevyengine.org/) game engine.

## Installation

To install this plugin, add the following to the `Cargo.toml`:

```toml
[dependencies]
bevy_debug_grid = "0.1"
```

## Setup

To use the plugin, import it by first doing `use bevy_debug_grid::*;` and then add the provided `DebugGridPlugin` plugin.

```rs
use bevy::prelude::*;
use bevy_debug_grid::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugGridPlugin::with_floor_grid())
        .run();
}
```

It is also possible to avoid spawning a default floor grid by doing `.add_plugin(DebugGridPlugin::without_floor_grid())`

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

All grids spawned by this plugin are meshes, rendered with a `polygoneMode` of `PolyginMode::Line`.  
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
    },
    TransformBundle::default(),
    VisibilityBundle::default(),
));
```

The `Grid::default()` is a small silver grid with 8 lines per axis and a spacing of `0.25_f32` between them.

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
        // Fills the remainging axis with None
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

## Known Bugs & Missing Features

- *Bug:* removing `TrackedGrid` or `GridAxis` will not properly update the other components. It will currently just break. Current workaround is to desapawn the entity.
- *Missing:* allowing grid tracking by custom means (it is by `With<Camera>` `query.get_single()` at the moment)
- *Missing:* grid color alpha

## Compatibility

| Bevy Version | Plugin Version |
|:------------:|:--------------:|
|    `0.10`    |     `0.1.0`    |

## License

This plugin is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
