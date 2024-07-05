use bevy::asset::load_internal_asset;
use bevy::prelude::*;
use std::marker::PhantomData;

use crate::*;

/// Spawns a default floor grid, resembling the one used in [Blender](https://www.blender.org/)
pub fn spawn_floor_grid(mut commands: Commands) {
    commands.spawn((
        Grid {
            spacing: 10.0_f32,
            count: 16,
            ..default()
        },
        SubGrid::default(),
        GridAxis::new_rgb(),
        TrackedGrid::default(),
        TransformBundle::default(),
        VisibilityBundle::default(),
    ));
}

/// The plugin which allows floor grids to work, where `T` is the component to track the floor grid to
pub struct TrackedDebugGridPlugin<T: Component> {
    spawn_floor_grid: bool,
    _phantom: PhantomData<T>,
}

impl<T: Component> TrackedDebugGridPlugin<T> {
    /// Adds the plugin along with a default floor grid
    pub const fn with_floor_grid() -> Self {
        Self {
            spawn_floor_grid: true,
            _phantom: PhantomData,
        }
    }

    /// Adds the plugin without spawning a default floor grid
    pub const fn without_floor_grid() -> Self {
        Self {
            spawn_floor_grid: false,
            _phantom: PhantomData,
        }
    }
}

impl<T: Component> Default for TrackedDebugGridPlugin<T> {
    fn default() -> Self {
        Self::with_floor_grid()
    }
}

impl<T: Component> Plugin for TrackedDebugGridPlugin<T> {
    fn build(&self, app: &mut App) {
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
            PreUpdate,
            (
                main_grid_mesher_untracked,
                main_grid_mesher_tracked,
                sub_grid_mesher,
                grid_axis_mesher,
                tracked_grid_updater::<T>,
                custom_tracked_grid_updater,
            ),
        )
        .add_systems(
            Update,
            (
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

pub type DebugGridPlugin = TrackedDebugGridPlugin<Camera>;
