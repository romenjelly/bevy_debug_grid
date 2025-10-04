use bevy::{prelude::*, color::palettes::tailwind};
use bevy_debug_grid::*;
use std::f32;

mod default_cube;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            default_cube::CameraControllerPlugin::from_positions(
                Vec3::new(-4.0_f32, 12.0_f32, 12.0_f32),
                Vec3::ZERO,
            ),
            DebugGridPlugin::with_floor_grid(),
        ))
        .add_systems(Startup, spawn_demonstration_objects)
        .add_systems(
            Update,
            (
                grid_changing_count,
                grid_changing_spacing,
                grid_changing_sub_count,
                grid_changing_color,
                grid_changing_sub_color,
            ),
        )
        .run();
}

fn spawn_demonstration_objects(mut commands: Commands) {
    let period = 4.0_f32;
    let spacing = f32::consts::TAU;
    let height_offset = f32::consts::FRAC_PI_2;
    let depth_offset: f32 = f32::consts::PI;

    // Changing count, red
    commands.spawn((
        Transform::from_xyz(-spacing, height_offset, -depth_offset),
        Visibility::default(),
        Grid {
            color: Color::Srgba(tailwind::RED_500),
            ..default()
        },
        GridChangePeriod(period),
        GridChangingCount { min: 1, max: 8 },
    ));
    // Changing spacing, green
    commands.spawn((
        Transform::from_xyz(0.0_f32, height_offset, -depth_offset),
        Visibility::default(),
        Grid {
            color: Color::Srgba(tailwind::GREEN_500),
            ..default()
        },
        GridChangePeriod(period),
        GridChangingSpacing {
            min: 0.25_f32,
            max: 0.5_f32,
        },
    ));
    // Changing count and spacing, blue
    commands.spawn((
        Transform::from_xyz(spacing, height_offset, -depth_offset),
        Visibility::default(),
        Grid {
            color: Color::Srgba(tailwind::BLUE_500),
            ..default()
        },
        GridChangePeriod(period),
        GridChangingCount { min: 1, max: 8 },
        GridChangingSpacing {
            min: 0.25_f32,
            max: 0.5_f32,
        },
    ));
    // Changing sub-count, cyan and magenta
    commands.spawn((
        Transform::from_xyz(-spacing, height_offset, depth_offset),
        Visibility::default(),
        Grid {
            color: Color::Srgba(tailwind::CYAN_500),
            ..default()
        },
        SubGrid {
            count: 4,
            color: Color::Srgba(tailwind::VIOLET_500),
        },
        GridChangePeriod(period),
        GridChangingSubCount { min: 0, max: 3 },
    ));
    // Changing color
    commands.spawn((
        Transform::from_xyz(0.0_f32, height_offset, depth_offset),
        Visibility::default(),
        Grid {
            color: Color::WHITE,
            ..default()
        },
        GridChangePeriod(period),
        GridChangingColor,
    ));
    // Changing sub-color, yellow
    commands.spawn((
        Transform::from_xyz(spacing, height_offset, depth_offset),
        Visibility::default(),
        Grid {
            color: Color::Srgba(tailwind::YELLOW_500),
            ..default()
        },
        SubGrid {
            count: 3,
            color: Color::WHITE,
        },
        GridChangePeriod(period),
        GridChangingSubColor,
    ));
}

#[derive(Component)]
struct GridChangePeriod(f32);

impl GridChangePeriod {
    /// Oscillation ranging in (0..=1.0) over a period of self.0
    fn oscillation(&self, time: &Time) -> f32 {
        (f32::sin(time.elapsed().as_secs_f32() * f32::consts::TAU / self.0) + 1.0_f32) * 0.5_f32
    }
    /// Inversion of `oscillation()`
    fn oscillation_inverted(&self, time: &Time) -> f32 {
        1.0_f32 - self.oscillation(time)
    }
}

#[derive(Component)]
struct GridChangingCount {
    min: usize,
    max: usize,
}

fn grid_changing_count(
    mut query: Query<(&mut Grid, &GridChangingCount, &GridChangePeriod)>,
    time: Res<Time>,
) {
    for (mut grid, count, period) in query.iter_mut() {
        let delta = count.max - count.min + 1; // +1 To adjust for oscillation upper bound
        let oscillation = period.oscillation(&time);
        grid.count = count.min + (delta as f32 * oscillation) as usize;
    }
}

#[derive(Component)]
struct GridChangingSpacing {
    min: f32,
    max: f32,
}

fn grid_changing_spacing(
    mut query: Query<(&mut Grid, &GridChangingSpacing, &GridChangePeriod)>,
    time: Res<Time>,
) {
    for (mut grid, count, period) in query.iter_mut() {
        let delta = count.max - count.min;
        let oscillation = period.oscillation_inverted(&time);
        grid.spacing = count.min + delta * oscillation;
    }
}

#[derive(Component)]
struct GridChangingSubCount {
    min: usize,
    max: usize,
}

fn grid_changing_sub_count(
    mut query: Query<(&mut SubGrid, &GridChangingSubCount, &GridChangePeriod)>,
    time: Res<Time>,
) {
    for (mut sub_grid, count, period) in query.iter_mut() {
        let delta = count.max - count.min + 1; // +1 To adjust for oscillation upper bound
        let oscillation = period.oscillation(&time);
        sub_grid.count = count.min + (delta as f32 * oscillation) as usize;
    }
}

#[derive(Component)]
struct GridChangingColor;

fn grid_changing_color(
    mut query: Query<(&mut Grid, &GridChangePeriod), With<GridChangingColor>>,
    time: Res<Time>,
) {
    for (mut grid, period) in query.iter_mut() {
        let oscillation = period.oscillation(&time);
        grid.color = Color::hsl(oscillation * 360.0_f32, 0.5_f32, 0.5_f32);
    }
}

#[derive(Component)]
struct GridChangingSubColor;

fn grid_changing_sub_color(
    mut query: Query<(&mut SubGrid, &GridChangePeriod), With<GridChangingSubColor>>,
    time: Res<Time>,
) {
    for (mut sub_grid, period) in query.iter_mut() {
        let oscillation = period.oscillation_inverted(&time);
        sub_grid.color = Color::hsl(oscillation * 360.0_f32, 0.5_f32, 0.5_f32);
    }
}
