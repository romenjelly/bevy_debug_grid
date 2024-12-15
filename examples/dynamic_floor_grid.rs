use bevy::{color::palettes::tailwind, prelude::*};
use bevy_debug_grid::*;
use bevy_spectator::*;

mod default_cube;

const COLOR_X_START: Srgba = tailwind::RED_500;
const COLOR_X_END: Srgba = tailwind::CYAN_500;

const COLOR_Y_START: Srgba = tailwind::GREEN_500;
const COLOR_Y_END: Srgba = tailwind::VIOLET_500;

const COLOR_Z_START: Srgba = tailwind::BLUE_500;
const COLOR_Z_END: Srgba = tailwind::YELLOW_500;

fn main() {
    App::new()
        .add_plugins((
            //DefaultPlugins,
            SpectatorPlugin,
            //DebugGridPlugin::without_floor_grid(),
        ))
        .add_systems(
            Startup,
            (
                spawn_floor_grid,
                default_cube::spawn_camera,
                spawn_center_sphere,
            ),
        )
        .add_systems(Update, (move_floor_grid, change_axis_color))
        .run();
}

fn spawn_center_sphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere::default())),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::WHITE))),
    ));
}

fn spawn_floor_grid(mut commands: Commands) {
    // Floor grid
    commands.spawn((
        Grid {
            spacing: 5.0_f32,
            count: 32,
            ..default()
        },
        SubGrid {
            count: 4,
            ..default()
        },
        GridAxis::new_empty(),
        TrackedGrid {
            alignment: GridAlignment::X,
            ..default()
        },
        Transform::default(),
        Visibility::default(),
    ));

    // Point light
    commands.spawn((
        Transform::from_xyz(4.0_f32, 4.0_f32, 4.0_f32),
        PointLight::default(),
    ));
}

fn move_floor_grid(mut query: Query<&mut TrackedGrid>, time: Res<Time>) {
    for mut grid in query.iter_mut() {
        grid.offset = time.elapsed_secs().sin();
    }
}

fn lerp_color(lhs: Srgba, rhs: Srgba, factor: f32) -> Color {
    let subbed = Srgba::rgb(
        rhs.red - lhs.red,
        rhs.green - lhs.green,
        rhs.blue - lhs.blue,
    );
    Color::Srgba(lhs + subbed * factor)
}

fn change_axis_color(mut query: Query<&mut GridAxis, With<TrackedGrid>>, time: Res<Time>) {
    let factor = ((time.elapsed_secs()).cos() + 1.0_f32) * 0.5_f32;
    for mut axis in query.iter_mut() {
        axis.x = Some(lerp_color(COLOR_X_START, COLOR_X_END, factor));
        axis.y = Some(lerp_color(COLOR_Y_START, COLOR_Y_END, factor));
        axis.z = Some(lerp_color(COLOR_Z_START, COLOR_Z_END, factor));
    }
}
