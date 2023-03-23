use bevy::prelude::*;

mod plugin;
mod rendering;
mod systems;

pub use plugin::*;
pub use rendering::*;
pub use systems::*;

/// The main grid component
#[derive(Component, Clone, Debug)]
pub struct Grid {
    /// Spacing between lines
    pub spacing: f32,
    /// Line count on one axis
    pub count: usize,
    /// Line color
    pub color: Color,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            spacing: 0.25_f32,
            count: 8,
            color: Color::SILVER,
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

/// Marker component to determine children spawned by a `SubGrid`
#[derive(Component)]
pub struct SubGridChild;

/// The tracking axis for a grid. *Ex:* `GridAlignment::Y` will result in a floor.
#[derive(Component, Default, Debug, Copy, Clone, PartialEq)]
pub enum GridAlignment {
    X,
    #[default]
    Y,
    Z,
}

impl GridAlignment {
    pub fn to_axis_vec3(&self) -> Vec3 {
        match self {
            GridAlignment::X => Vec3::X,
            GridAlignment::Y => Vec3::Y,
            GridAlignment::Z => Vec3::Z,
        }
    }

    pub fn to_inverted_axis_vec3(&self) -> Vec3 {
        Vec3::ONE - self.to_axis_vec3()
    }

    /// Shifts/rotates a `Vec3`'s values. Default `Y` alignment does nothing.
    pub fn shift_vec3(&self, input: Vec3) -> Vec3 {
        match self {
            Self::X => Vec3::new(input.y, input.z, input.x),
            Self::Y => input,
            Self::Z => Vec3::new(input.z, input.x, input.y),
        }
    }
}

impl Eq for GridAlignment {}

impl From<GridAlignment> for Vec3 {
    fn from(val: GridAlignment) -> Self {
        val.to_inverted_axis_vec3()
    }
}

/// Custom color overrides for axis of a grid.
/// Spawn it next to a grid for it to have effect.
#[derive(Component, Clone, Debug)]
pub struct GridAxis {
    pub x: Option<Color>,
    pub y: Option<Color>,
    pub z: Option<Color>,
}

impl GridAxis {
    /// An empty grid axis, does nothing.
    /// Use for later mutation or debug.
    pub fn new_empty() -> Self {
        Self {
            x: None,
            y: None,
            z: None,
        }
    }

    /// Creates a grid axis with each axis having a color.
    /// Red for X, green for Y, and blue for Z.
    pub fn new_rgb() -> Self {
        Self {
            x: Some(Color::RED),
            y: Some(Color::GREEN),
            z: Some(Color::BLUE),
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
    pub fn default_axis() -> [GridAlignment; 2] {
        [
            GridAlignment::X,
            GridAlignment::Z,
        ]
    }

    /// Returns an axis color by grid alignment, if such a color is configured per that axis
    pub fn get_by_alignment(&self, alignment: &GridAlignment) -> Option<Color> {
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
#[derive(Component, Clone, Debug, Default)]
pub struct TrackedGrid {
    /// The axis on which the grid will be tracked
    pub alignment: GridAlignment,
    /// The offset the grid has in relation to its tracking axis
    pub offset: f32,
}
