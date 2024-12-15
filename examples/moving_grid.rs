use bevy::{color::palettes::tailwind, prelude::*};
use bevy_debug_grid::*;
use bevy_spectator::*;
use std::f32;

mod changing_grid;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            SpectatorPlugin,
            DebugGridPlugin::with_floor_grid(),
        ))
        .add_systems(
            Startup,
            (changing_grid::spawn_camera, spawn_demonstration_objects),
        )
        .add_systems(Update, (move_objects, spin_objects))
        .run();
}

#[derive(Component)]
struct Moving {
    origin: Vec3,
}

#[derive(Component)]
struct Spinning;

fn spawn_demonstration_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let spacing_offset = f32::consts::TAU;
    let height_offset = f32::consts::FRAC_PI_2;
    let depth_offset: f32 = f32::consts::PI;

    // Moving mesh, red
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::WHITE))),
        Moving {
            origin: Vec3::new(-spacing_offset, height_offset, -depth_offset),
        },
        Grid {
            color: Color::Srgba(tailwind::RED_500),
            ..default()
        },
    ));
    // Spinning mesh, green
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::WHITE))),
        Transform::from_xyz(0.0_f32, height_offset, -depth_offset),
        Grid {
            color: Color::Srgba(tailwind::GREEN_500),
            ..default()
        },
        Spinning,
    ));
    // Moving and spinning mesh, blue
    commands.spawn((
        Mesh3d(meshes.add(Torus::default())),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::WHITE))),
        Grid {
            color: Color::Srgba(tailwind::BLUE_500),
            ..default()
        },
        Moving {
            origin: Vec3::new(spacing_offset, height_offset, -depth_offset),
        },
        Spinning,
    ));
    // Moving grid, cyan
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(StandardMaterial::from(Color::WHITE))),
            Transform::from_xyz(-spacing_offset, height_offset, depth_offset),
        ))
        .with_children(|child| {
            child.spawn((
                Transform::default(),
                Visibility::default(),
                Grid {
                    color: Color::Srgba(tailwind::CYAN_500),
                    ..default()
                },
                Moving { origin: Vec3::ZERO },
            ));
        });
    // Spinning grid, magenta
    commands
        .spawn((
            Mesh3d(meshes.add(Capsule3d::default())),
            MeshMaterial3d(materials.add(StandardMaterial::from(Color::WHITE))),
            Transform::from_xyz(0.0_f32, height_offset, depth_offset),
        ))
        .with_children(|child| {
            child.spawn((
                Transform::default(),
                Visibility::default(),
                Grid {
                    color: Color::Srgba(tailwind::VIOLET_500),
                    ..default()
                },
                Spinning,
            ));
        });
    // Moving and spinning grid, yellow
    commands
        .spawn((
            Mesh3d(meshes.add(Torus::default())),
            MeshMaterial3d(materials.add(StandardMaterial::from(Color::WHITE))),
            Transform::from_xyz(spacing_offset, height_offset, depth_offset),
        ))
        .with_children(|child| {
            child.spawn((
                Transform::default(),
                Visibility::default(),
                Grid {
                    color: Color::Srgba(tailwind::YELLOW_500),
                    ..default()
                },
                Moving { origin: Vec3::ZERO },
                Spinning,
            ));
        });

    // Point light
    commands.spawn((
        Transform::from_xyz(4.0_f32, 4.0_f32, 4.0_f32),
        PointLight::default(),
    ));
}

fn move_objects(mut query: Query<(&mut Transform, &Moving)>, time: Res<Time>) {
    for (mut transform, moving) in query.iter_mut() {
        let elapsed = time.elapsed().as_secs_f32();
        transform.translation = moving.origin
            + Vec3::new(
                f32::sin(elapsed + f32::cos(elapsed)),
                f32::cos(elapsed + f32::sin(elapsed)),
                f32::cos(elapsed * 0.5_f32),
            );
    }
}

fn spin_objects(mut query: Query<&mut Transform, With<Spinning>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        let elapsed = time.elapsed().as_secs_f32();
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            f32::sin(elapsed + f32::cos(elapsed)),
            f32::cos(elapsed + f32::sin(elapsed)),
            f32::cos(elapsed * 0.5_f32),
        );
    }
}
