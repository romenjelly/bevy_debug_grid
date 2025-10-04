use bevy::{
    prelude::*,
    camera::{visibility::{Layer, RenderLayers}, RenderTarget},
    color::palettes::tailwind,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};
use bevy_debug_grid::*;

mod default_cube;
use default_cube::*;

/**
 * This example demonstrates the usage of render layers, and custom tracking overrides for grids.
 *
 * The setup system will spawn a cube, 1 main camera, 2 grids, and 2 secondary cameras.
 * The main camera is able to be flown around with the spectator plugin.
 * The secondary cameras both see different render layers, and a differing grid is contained on each said layer.
 * Secondary camera outputs are rendered onto cubes attached to the main camera, making them always visible.
 *   Top-most camera: renders the cube from directly above
 *   Bottom-most camera: renders the cube from a diagonal perspective
 * The bottom-most of those cubes has a tracked grid rendered onto it, making use of the tracking override
 */

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraControllerPlugin::without_camera(),
            // Since this world has 3 cameras, we tell the grid plugin to track the entity with the Spectator component
            TrackedDebugGridPlugin::<ControlledCamera>::with_floor_grid(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, floating_object)
        .run();
}

/// Creates a 256x256 render texture and returns the handle
fn create_render_texture(images: &mut Assets<Image>) -> Handle<Image> {
    let size: Extent3d = Extent3d {
        width: 256,
        height: 256,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);

    images.add(image)
}

// Spins, and moves an entity up and down
#[derive(Component)]
struct Floating;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // The render layer used for the top render texture
    const TOP_LAYER: Layer = 1;
    let top_image_handle = create_render_texture(&mut images);

    // Top render layer camera
    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::Srgba(tailwind::GRAY_500)),
            order: -1,
            target: RenderTarget::Image(top_image_handle.clone().into()),
            ..default()
        },
        Transform::from_xyz(0.0_f32, 8.0_f32, 0.0_f32).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::layer(TOP_LAYER),
    ));

    // A grid visible only on the top render layer
    commands.spawn((
        Grid {
            color: Color::Srgba(tailwind::ORANGE_500.with_alpha(Grid::DEFAULT_ALPHA)),
            ..default()
        },
        Visibility::default(),
        Transform::default(),
        RenderLayers::layer(TOP_LAYER),
    ));

    const BOTTOM_LAYER: Layer = 2;
    let bottom_image_handle = create_render_texture(&mut images);

    // Bottom render layer camera
    // The entity id is saved to later pass it to the grid tracking override
    let secondary_camera_entity = commands
        .spawn((
            Camera3d::default(),
            Camera {
                clear_color: ClearColorConfig::Custom(Color::Srgba(tailwind::GRAY_500)),
                order: -1,
                target: RenderTarget::Image(bottom_image_handle.clone().into()),
                ..default()
            },
            Transform::from_xyz(-4.0_f32, 2.0_f32, 4.0_f32).looking_at(Vec3::Y, Vec3::Y),
            RenderLayers::layer(BOTTOM_LAYER),
        ))
        .id();

    // A tracked grid visible on the bottom render layer
    commands.spawn((
        Grid {
            count: 6,
            spacing: 5.0_f32,
            color: Color::Srgba(tailwind::CYAN_500.with_alpha(Grid::DEFAULT_ALPHA)),
            ..default()
        },
        SubGrid {
            count: 9,
            color: Color::WHITE.with_alpha(Grid::DEFAULT_ALPHA),
        },
        TrackedGrid {
            // It is tracked to the secondary camera entity instead of the entity containing a Spectator component
            tracking_override: Some(secondary_camera_entity),
            ..default()
        },
        Visibility::default(),
        Transform::default(),
        RenderLayers::layer(BOTTOM_LAYER),
    ));

    // Cube in the center
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0_f32, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Transform::from_xyz(0.0_f32, 0.75_f32, 0.0_f32),
        Floating,
        // Make the cube visible on all relevant render layers
        RenderLayers::default().with(TOP_LAYER).with(BOTTOM_LAYER),
    ));

    // Lighting
    commands.spawn((
        PointLight::default(),
        Transform::from_translation(Vec3::splat(8.0_f32)),
    ));

    // Main render pass camera with parented render textures
    commands
        .spawn(camera_bundle(CameraControllerPlugin::default_transform()))
        .with_children(|parent| {
            // Top render texture, looking at a grid
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.25_f32, 0.25, 0.25))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color_texture: Some(top_image_handle),
                    unlit: true,
                    double_sided: true,
                    ..default()
                })),
                Transform::from_xyz(0.5_f32, 0.2_f32, -1.0_f32).looking_at(Vec3::ZERO, Vec3::Y),
            ));
            // Bottom render texture, looking at a tracked grid
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.25_f32, 0.25, 0.25))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color_texture: Some(bottom_image_handle),
                    unlit: true,
                    double_sided: true,
                    ..default()
                })),
                Transform::from_xyz(0.5_f32, -0.2_f32, -1.0_f32).looking_at(Vec3::ZERO, Vec3::Y),
            ));
        });
}

// System for moving the entities with the Floating component
fn floating_object(time: Res<Time>, mut query: Query<&mut Transform, With<Floating>>) {
    for mut transform in &mut query {
        transform.translation.y += (time.elapsed_secs() * 2.0_f32).sin() * 0.004_f32;
        transform.rotate_y(time.delta_secs() * 0.75_f32);
    }
}
