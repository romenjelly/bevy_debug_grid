use bevy::{color::palettes::tailwind, prelude::*};

mod plugin;
pub mod rendering;
pub mod systems;

pub use plugin::*;
use rendering::*;
use systems::*;

pub mod prelude {
    pub use crate::plugin::*;
    pub use super::{
        Grid,
        SubGrid,
        GridAlignment,
        GridAxis,
        TrackedGrid,
    };
}

/// The main grid component
#[derive(Component, Clone, Debug)]
pub struct Grid {
    /// Spacing between lines
    pub spacing: f32,
    /// Line count on one axis
    pub count: usize,
    /// Line color
    pub color: Color,
    /// Alpha mode
    pub alpha_mode: AlphaMode,
}

impl Grid {
    pub const DEFAULT_SRGBA: Srgba = tailwind::GRAY_400;
    pub const DEFAULT_ALPHA: f32 = 0.5_f32;
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            spacing: 0.25_f32,
            count: 8,
            color: Color::Srgba(Self::DEFAULT_SRGBA.with_alpha(Self::DEFAULT_ALPHA)),
            alpha_mode: AlphaMode::Blend,
        }
    }
}

/// Marker component to determine children spawned by a `Grid`
#[derive(Component)]
pub struct GridChild;

/// The sub-grid component, adds lines between the lines of a grid.
/// Spawn it next to a grid for it to have effect.
#[derive(Component, Clone, Debug)]
pub struct SubGrid {
    /// Line count between the main grid's lines
    pub count: usize,
    /// Line color
    pub color: Color,
}

impl SubGrid {
    pub const DEFAULT_SRGBA: Srgba = tailwind::GRAY_500;
}

impl Default for SubGrid {
    fn default() -> Self {
        Self {
            count: 9,
            color: Color::Srgba(Self::DEFAULT_SRGBA.with_alpha(Grid::DEFAULT_ALPHA)),
        }
    }
}

/// Marker component to determine children spawned by a `SubGrid`
#[derive(Component)]
pub struct SubGridChild;

/// The tracking axis for a grid. *Ex:* `GridAlignment::Y` will result in a floor.
#[derive(Component, Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum GridAlignment {
    X,
    #[default]
    Y,
    Z,
}

impl GridAlignment {
    pub const fn to_axis_vec3(&self) -> Vec3 {
        match self {
            Self::X => Vec3::X,
            Self::Y => Vec3::Y,
            Self::Z => Vec3::Z,
        }
    }

    pub fn to_inverted_axis_vec3(&self) -> Vec3 {
        Vec3::ONE - self.to_axis_vec3()
    }

    /// Shifts/rotates a `Vec3`'s values. Default `Y` alignment does nothing.
    pub const fn shift_vec3(&self, input: Vec3) -> Vec3 {
        match self {
            Self::X => Vec3::new(input.y, input.z, input.x),
            Self::Y => input,
            Self::Z => Vec3::new(input.z, input.x, input.y),
        }
    }
}

impl From<GridAlignment> for Vec3 {
    fn from(val: GridAlignment) -> Self {
        val.to_inverted_axis_vec3()
    }
}

/// Custom color overrides for axis of a grid.
/// Spawn it next to a grid for it to have effect.
#[derive(Component, Clone, Debug)]
pub struct GridAxis {
    /// Color of the X axis
    pub x: Option<Color>,
    /// Color of the Y axis
    pub y: Option<Color>,
    /// Color of the Z axis
    pub z: Option<Color>,
}

impl GridAxis {
    /// An empty grid axis, does nothing.
    /// Use for later mutation or debug.
    pub const fn new_empty() -> Self {
        Self {
            x: None,
            y: None,
            z: None,
        }
    }

    /// Creates a grid axis with each axis having a color.
    /// Red for X, green for Y, and blue for Z.
    pub const fn new_rgb() -> Self {
        Self {
            x: Some(Color::Srgba(tailwind::RED_500)),
            y: Some(Color::Srgba(tailwind::GREEN_500)),
            z: Some(Color::Srgba(tailwind::BLUE_500)),
        }
    }

    /// Creates a single axis mesh consisting of two `Vec3`s
    pub fn create_single_axis(size: f32, alignment: GridAlignment) -> [Vec3; 2] {
        [
            alignment.shift_vec3(Vec3::new(0.0_f32, size, 0.0_f32)),
            alignment.shift_vec3(Vec3::new(0.0_f32, -size, 0.0_f32)),
        ]
    }

    /// Creates grid axis from the configured colors.
    /// Returns a vector of used axis with their corresponding color, as well as a vector of unused axis, `(used, unused)`.
    pub fn create_axis(&self) -> (Vec<(GridAlignment, Color)>, Vec<GridAlignment>) {
        let mut axis = Vec::new();
        let mut unused = Vec::new();

        if let Some(color) = self.x {
            axis.push((GridAlignment::X, color));
        } else {
            unused.push(GridAlignment::X);
        }
        // Y axis does not create a default
        if let Some(color) = self.y {
            axis.push((GridAlignment::Y, color));
        }
        if let Some(color) = self.z {
            axis.push((GridAlignment::Z, color));
        } else {
            unused.push(GridAlignment::Z);
        }

        (axis, unused)
    }

    /// Returns the default axis for a grid
    pub const fn default_axis() -> [GridAlignment; 2] {
        [GridAlignment::X, GridAlignment::Z]
    }

    /// Returns an axis color by grid alignment, if such a color is configured per that axis
    pub const fn get_by_alignment(&self, alignment: &GridAlignment) -> Option<Color> {
        match alignment {
            GridAlignment::X => self.x,
            GridAlignment::Y => self.y,
            GridAlignment::Z => self.z,
        }
    }
}

impl Default for GridAxis {
    fn default() -> Self {
        Self::new_empty()
    }
}

/// Marker component to determine children spawned by a `GridAxis`
#[derive(Component)]
pub struct GridAxisChild;

/// Marks a grid as "tracked", meaning it will move with the main camera
///
/// Note: A tracked grid should not be parented to a moving entity.
#[derive(Component, Clone, Debug, Default)]
pub struct TrackedGrid {
    /// The axis on which the grid will be tracked
    pub alignment: GridAlignment,
    /// The offset the grid has in relation to its tracking axis
    pub offset: f32,
    /// Entity to be tracked instead of the plugin's generic component
    pub tracking_override: Option<Entity>,
}
