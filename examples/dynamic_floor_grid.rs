use bevy::prelude::*;
use bevy_spectator::*;
use bevy_debug_grid::*;

const COLOR_X_START: Color = Color::RED;
const COLOR_X_END: Color = Color::CYAN;

const COLOR_Y_START: Color = Color::GREEN;
const COLOR_Y_END: Color = Color::rgb(1.0_f32, 0.0_f32, 1.0_f32);  // Magenta

const COLOR_Z_START: Color = Color::BLUE;
const COLOR_Z_END: Color = Color::YELLOW;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            SpectatorPlugin,
            DebugGridPlugin::without_floor_grid(),
        ))
        .add_systems(Startup, (
            spawn_floor_grid,
            spawn_camera,
            spawn_center_sphere,
        ))
        .add_systems(Update, (
            move_floor_grid,
            change_axis_color,
        ))
        .run();
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle::default(),
        Spectator,
    ));
}

fn spawn_center_sphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Icosphere::default().try_into().expect("Unable to mesh default icosphere")),
        material: materials.add(Color::WHITE.into()),
        ..default()
    });
}

fn spawn_floor_grid(mut commands: Commands) {
    // Floor grid
    commands.spawn((
        Grid {
            spacing: 5.0_f32,
            count: 32,
            color: Color::SILVER,
            ..default()
        },
        SubGrid {
            count: 4,
            color: Color::GRAY,
        },
        GridAxis::new_empty(),
        TrackedGrid {
            alignment: GridAlignment::X,
            ..default()
        },
        TransformBundle::default(),
        VisibilityBundle::default(),
    ));

    // Point light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0_f32, 4.0_f32, 4.0_f32),
        ..default()
    });
}

fn move_floor_grid(
    mut query: Query<&mut TrackedGrid>,
    time: Res<Time>,
) {
    for mut grid in query.iter_mut() {
        grid.offset = time.elapsed_seconds().sin();
    }
}

fn lerp_color(lhs: Color, rhs: Color, factor: f32) -> Color {
    let subbed = Color::rgb(rhs.r() - lhs.r(), rhs.g() - lhs.g(), rhs.b() - lhs.b());
    lhs + subbed * factor
}

fn change_axis_color(
    mut query: Query<&mut GridAxis, With<TrackedGrid>>,
    time: Res<Time>,
) {
    let factor = ((time.elapsed_seconds()).cos() + 1.0_f32) * 0.5_f32;
    for mut axis in query.iter_mut() {
        axis.x = Some(lerp_color(COLOR_X_START, COLOR_X_END, factor));
        axis.y = Some(lerp_color(COLOR_Y_START, COLOR_Y_END, factor));
        axis.z = Some(lerp_color(COLOR_Z_START, COLOR_Z_END, factor));
    }
}
