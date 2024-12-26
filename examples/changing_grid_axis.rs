use bevy::prelude::*;
use bevy_debug_grid::*;
use bevy_spectator::*;

mod default_cube;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            SpectatorPlugin,
            DebugGridPlugin::without_floor_grid(),
        ))
        .add_systems(Startup, (default_cube::spawn_camera, spawn_demonstration_grid))
        .add_systems(Update, change_axis_color)
        .run();
}

#[derive(Component)]
struct ChangingAxis;

fn spawn_demonstration_grid(mut commands: Commands) {
    commands.spawn((
        Grid::default(),
        GridAxis::new_empty(),
        ChangingAxis,
        Transform::default(),
        Visibility::default(),
    ));
}

fn change_axis_color(mut query: Query<&mut GridAxis, With<ChangingAxis>>, time: Res<Time>) {
    let elapsed = time.elapsed_secs() * 0.25_f32;
    let selected_axis = (elapsed % 4.0_f32) as usize;
    let axis_color = Some(Color::hsl(
        (elapsed % 1.0_f32) * 360.0_f32,
        0.5_f32,
        0.5_f32,
    ));
    for mut axis in query.iter_mut() {
        axis.x = None;
        axis.y = None;
        axis.z = None;
        match selected_axis {
            0 => axis.x = axis_color,
            1 => axis.y = axis_color,
            2 => axis.z = axis_color,
            _ => {}
        };
    }
}
