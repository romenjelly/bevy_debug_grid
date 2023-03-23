use bevy::prelude::*;
use bevy_spectator::*;
use bevy_debug_grid::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpectatorPlugin)
        .add_plugin(DebugGridPlugin::with_floor_grid())
        .add_startup_system(spawn_camera)
        .add_startup_system(default_cube)
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

fn default_cube(
    mut commands: Commands,
    mut assets: ResMut<Assets<Mesh>>,
) {
    commands.spawn(PbrBundle {
        mesh: assets.add(shape::Cube::new(1.0_f32).into()),
        transform: Transform::from_xyz(0.0_f32, 0.5_f32, 0.0_f32),
        ..default()
    });
}
