use bevy::prelude::*;
use bevy_debug_grid::*;
use bevy_spectator::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpectatorPlugin)
        .add_plugin(DebugGridPlugin::with_floor_grid())
        .add_startup_system(spawn_camera)
        .add_startup_system(default_cube)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle::default(), Spectator));
}

fn default_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Cube::new(1.0_f32).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_xyz(0.0_f32, 0.5_f32, 0.0_f32),
        ..default()
    });

    // Point light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0_f32, 4.0_f32, 4.0_f32),
        ..default()
    });
}
