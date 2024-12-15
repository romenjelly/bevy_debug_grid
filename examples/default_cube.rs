use bevy::prelude::*;
use bevy_debug_grid::*;
use bevy_spectator::*;

#[allow(dead_code)]

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            SpectatorPlugin,
            DebugGridPlugin::with_floor_grid(),
        ))
        .add_systems(Startup, (spawn_camera, default_cube))
        .run();
}

pub fn camera_bundle() -> impl Bundle {
    (
        Transform::from_xyz(7.0_f32, 3.5_f32, 4.0_f32).looking_at(Vec3::Y * 0.5_f32, Vec3::Y),
        Camera3d::default(),
        Spectator,
    )
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(camera_bundle());
}

fn default_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0_f32, 1.0_f32, 1.0_f32))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::WHITE))),
        Transform::from_xyz(0.0_f32, 0.5_f32, 0.0_f32),
    ));

    // Point light
    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(4.0_f32, 4.0_f32, 4.0_f32),
    ));
}
