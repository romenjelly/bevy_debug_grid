use bevy::{
    asset::load_internal_asset,
    prelude::{MaterialPlugin, Plugin},
};

use crate::*;

/// Spawns a default floor grid, resembing the one used in [Blender](https://www.blender.org/)
pub fn spawn_floor_grid(mut commands: Commands) {
    commands.spawn((
        Grid {
            spacing: 10.0_f32,
            count: 16,
            color: Color::SILVER.with_a(DEFAULT_GRID_ALPHA),
            alpha_mode: AlphaMode::Blend,
        },
        SubGrid {
            count: 9,
            color: Color::GRAY.with_a(DEFAULT_GRID_ALPHA),
        },
        GridAxis::new_rgb(),
        TrackedGrid::default(),
        TransformBundle::default(),
        VisibilityBundle::default(),
    ));
}

/// The plugin which allows floor grids to work
pub struct DebugGridPlugin {
    spawn_floor_grid: bool,
}

impl DebugGridPlugin {
    /// Adds the plugin along with a default floor grid
    pub fn with_floor_grid() -> Self {
        Self {
            spawn_floor_grid: true,
        }
    }

    /// Adds the plugin without spawning a default floor grid
    pub fn without_floor_grid() -> Self {
        Self {
            spawn_floor_grid: false,
        }
    }
}

impl Default for DebugGridPlugin {
    fn default() -> Self {
        Self::with_floor_grid()
    }
}

impl Plugin for DebugGridPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        load_internal_asset!(
            app,
            CLIPPED_LINE_SHADER_HANDLE,
            "shaders/clipped_line.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            SIMPLE_LINE_SHADER_HANDLE,
            "shaders/simple_line.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins((
            MaterialPlugin::<SimpleLineMaterial>::default(),
            MaterialPlugin::<ClippedLineMaterial>::default(),
        ))
        .add_systems(
            Update,
            (
                main_grid_mesher_untracked,
                main_grid_mesher_tracked,
                sub_grid_mesher,
                grid_axis_mesher,
                floor_grid_updater,
                despawn_children_upon_removal::<Grid, GridChild>,
                despawn_children_upon_removal::<Grid, SubGridChild>,
                despawn_children_upon_removal::<Grid, GridAxisChild>,
                despawn_children_upon_removal::<SubGrid, SubGridChild>,
                despawn_children_upon_removal::<GridAxis, GridAxisChild>,
            ),
        );

        if self.spawn_floor_grid {
            app.add_systems(Startup, spawn_floor_grid);
        }
    }
}
