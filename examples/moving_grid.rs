use bevy::prelude::*;
use bevy_spectator::*;
use bevy_debug_grid::*;
use std::f32;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpectatorPlugin)
        .add_plugin(DebugGridPlugin::with_floor_grid())
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_demonstration_objects)
        .add_system(move_objects)
        .add_system(spin_objects)
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

#[derive(Component)]
struct Moving {
    origin: Vec3,
}

#[derive(Component)]
struct Spinning;

fn spawn_demonstration_objects(
    mut commands: Commands,
    mut assets: ResMut<Assets<Mesh>>,
) {
    let spacing_offset = f32::consts::TAU;
    let height_offset = f32::consts::FRAC_PI_2;
    let depth_offset: f32 = f32::consts::PI;

    // Moving mesh, red
    commands.spawn((
        PbrBundle {
            mesh: assets.add(shape::Cube::default().into()),
            ..default()
        },
        Moving {
            origin: Vec3::new(-spacing_offset, height_offset, -depth_offset),
        },
        Grid {
            color: Color::RED,
            ..default()
        },
    ));
    // Spinning mesh, green
    commands.spawn((
        PbrBundle {
            mesh: assets.add(shape::Capsule::default().into()),
            transform: Transform::from_xyz(0.0_f32, height_offset, -depth_offset),
            ..default()
        },
        Grid {
            color: Color::GREEN,
            ..default()
        },
        Spinning,
    ));
    // Moving and spinning mesh, blue
    commands.spawn((
        PbrBundle {
            mesh: assets.add(shape::Torus::default().into()),
            ..default()
        },
        Grid {
            color: Color::BLUE,
            ..default()
        },
        Moving {
            origin: Vec3::new(spacing_offset, height_offset, -depth_offset),
        },
        Spinning,
    ));
    // Moving grid, cyan
    commands.spawn(PbrBundle {
        mesh: assets.add(shape::Cube::default().into()),
        transform: Transform::from_xyz(-spacing_offset, height_offset, depth_offset),
        ..default()
    }).with_children(|child| {
        child.spawn((
            TransformBundle::default(),
            VisibilityBundle::default(),
            Grid {
                color: Color::CYAN,
                ..default()
            },
            Moving { origin: Vec3::ZERO },
        ));
    });
    // Spinning grid, magenta
    commands.spawn(PbrBundle {
        mesh: assets.add(shape::Capsule::default().into()),
        transform: Transform::from_xyz(0.0_f32, height_offset, depth_offset),
        ..default()
    }).with_children(|child| {
        child.spawn((
            TransformBundle::default(),
            VisibilityBundle::default(),
            Grid {
                color: Color::rgb(1.0_f32, 0.0_f32, 1.0_f32),  // Magenta
                ..default()
            },
            Spinning,
        ));
    });
    // Moving and spinning grid, yellow
    commands.spawn(PbrBundle {
        mesh: assets.add(shape::Torus::default().into()),
        transform: Transform::from_xyz(spacing_offset, height_offset, depth_offset),
        ..default()
    }).with_children(|child| {
        child.spawn((
            TransformBundle::default(),
            VisibilityBundle::default(),
            Grid {
                color: Color::YELLOW,
                ..default()
            },
            Moving { origin: Vec3::ZERO },
            Spinning,
        ));
    });
}

fn move_objects(
    mut query: Query<(&mut Transform, &Moving)>,
    time: Res<Time>,
) {
    for (mut transform, moving) in query.iter_mut() {
        let elapsed = time.elapsed().as_secs_f32();
        transform.translation = moving.origin + Vec3::new(
            f32::sin(elapsed + f32::cos(elapsed)),
            f32::cos(elapsed + f32::sin(elapsed)),
            f32::cos(elapsed * 0.5_f32),
        );
    }
}

fn spin_objects(
    mut query: Query<&mut Transform, With<Spinning>>,
    time: Res<Time>,
) {
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
