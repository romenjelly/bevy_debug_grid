use bevy::prelude::*;
use bevy_debug_grid::*;

#[allow(unused_imports)]
pub use camera_controller::{camera_bundle, CameraControllerPlugin, ControlledCamera};

#[allow(dead_code)]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraControllerPlugin::default(),
            DebugGridPlugin::with_floor_grid(),
        ))
        .add_systems(Startup, default_cube)
        .run();
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
        Visibility::default(),
    ));

    // Point light
    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(4.0_f32, 4.0_f32, 4.0_f32),
    ));
}

// Camera controller used by every example
pub mod camera_controller {
    use bevy::input::mouse::MouseMotion;
    use bevy::prelude::*;
    use bevy::window::CursorOptions;

    #[derive(Component)]
    pub struct ControlledCamera;

    #[derive(Component)]
    pub struct FocusedControlledCamera;

    pub struct CameraControllerPlugin {
        pub camera_transform: Option<Transform>,
    }

    #[allow(dead_code)]
    impl CameraControllerPlugin {
        pub const DEFAULT_CAMERA_ORIGIN: Vec3 = Vec3::new(7.0_f32, 3.5_f32, 4.0_f32);
        pub const DEFAULT_CAMERA_LOOK_AT: Vec3 = Vec3::new(0.0_f32, 0.5_f32, 0.0_f32);

        pub fn without_camera() -> Self {
            Self {
                camera_transform: None,
            }
        }

        pub fn with_camera(camera_transform: Transform) -> Self {
            Self {
                camera_transform: Some(camera_transform),
            }
        }

        pub fn from_positions(origin: Vec3, look_at: Vec3) -> Self {
            Self::with_camera(Transform::from_translation(origin).looking_at(look_at, Vec3::Y))
        }

        pub fn default_transform() -> Transform {
            Transform::from_translation(Self::DEFAULT_CAMERA_ORIGIN)
                .looking_at(Self::DEFAULT_CAMERA_LOOK_AT, Vec3::Y)
        }
    }

    impl Default for CameraControllerPlugin {
        fn default() -> Self {
            Self::with_camera(Self::default_transform())
        }
    }

    impl Plugin for CameraControllerPlugin {
        fn build(&self, app: &mut App) {
            if let Some(camera_transform) = self.camera_transform {
                app.add_systems(Startup, spawn_camera(camera_transform));
            }
            app.add_systems(Update, (handle_focus, translate_camera, rotate_camera));
        }
    }

    pub fn camera_bundle(camera_transform: Transform) -> impl Bundle {
        (Camera3d::default(), ControlledCamera, camera_transform)
    }

    fn spawn_camera(camera_transform: Transform) -> impl Fn(Commands) {
        move |mut commands| {
            commands.spawn(camera_bundle(camera_transform));
        }
    }

    fn handle_focus(
        mut commands: Commands,
        entity: Single<Entity, With<ControlledCamera>>,
        mut cursor_options: Single<&mut CursorOptions>,
        mouse: Res<ButtonInput<MouseButton>>,
        key: Res<ButtonInput<KeyCode>>,
    ) {
        if mouse.just_pressed(MouseButton::Left) {
            cursor_options.visible = false;
            cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
            commands.entity(*entity).insert(FocusedControlledCamera);
        }

        if key.just_pressed(KeyCode::Escape) {
            cursor_options.visible = true;
            cursor_options.grab_mode = bevy::window::CursorGrabMode::None;
            commands.entity(*entity).remove::<FocusedControlledCamera>();
        }
    }

    fn translate_camera(
        time: Res<Time>,
        mut transform: Single<&mut Transform, With<FocusedControlledCamera>>,
        keys: Res<ButtonInput<KeyCode>>,
    ) {
        let axis_motion_intent = |positive: KeyCode, negative: KeyCode| {
            (keys.pressed(positive) as i8 - keys.pressed(negative) as i8) as f32
        };

        let speed = if !keys.pressed(KeyCode::ShiftLeft) {
            2.0_f32
        } else {
            8.0_f32
        };

        let mut translation_intent = transform.rotation
            * Vec3::new(
                axis_motion_intent(KeyCode::KeyD, KeyCode::KeyA),
                0.0_f32,
                axis_motion_intent(KeyCode::KeyS, KeyCode::KeyW),
            );
        translation_intent.y = 0.0_f32;
        translation_intent = translation_intent.normalize_or_zero();
        translation_intent.y = axis_motion_intent(KeyCode::Space, KeyCode::ControlLeft);

        transform.translation += translation_intent * speed * time.delta_secs();
    }

    fn rotate_camera(
        mut transform: Single<&mut Transform, With<FocusedControlledCamera>>,
        mut motion: MessageReader<MouseMotion>,
    ) {
        let max_angle = 89.0_f32.to_radians();
        let sensitivity = 0.001_f32;
        let delta = motion.read().map(|motion| motion.delta).sum::<Vec2>() * -sensitivity;

        let (x, y, ..) = transform.rotation.to_euler(EulerRot::YXZ);
        transform.rotation = Quat::from_euler(
            EulerRot::YXZ,
            x + delta.x,
            f32::clamp(y + delta.y, -max_angle, max_angle),
            0.0_f32,
        );
    }
}
