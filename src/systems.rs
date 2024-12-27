#![allow(clippy::type_complexity)]

use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::view::RenderLayers;
use bevy::utils::HashMap;

use crate::*;

/// Offset on the vertical axis for a sub-grid.
/// Purely used to avoid Z-fighting.
/// The mesh is offset by it, and the mesh's transform is offset by it as well.
/// Can be any reasonable float value.
const SUB_GRID_VERTICAL_OFFSET: f32 = -0.001_f32;

/// Utility function to despawn children of a certain type.
/// Used with marker components.
fn despawn_children_of_type<T: Component>(
    commands: &mut Commands,
    parent: Entity,
    children: &Children,
    query: &Query<Entity, With<T>>,
) {
    let children = children
        .into_iter()
        .filter_map(|child| query.get(*child).ok())
        .collect::<Vec<_>>();
    commands.entity(parent).remove_children(&children);
    for child in children {
        commands.entity(child).despawn();
    }
}

/// Creates vertices for a line based on the line's size and its offset
fn line_vertices(size: f32, horizontal_offset: f32, vertical_offset: f32) -> [Vec3; 8] {
    [
        // +X line
        Vec3::new(horizontal_offset, vertical_offset, size),
        Vec3::new(horizontal_offset, vertical_offset, -size),
        // -X line
        Vec3::new(-horizontal_offset, vertical_offset, size),
        Vec3::new(-horizontal_offset, vertical_offset, -size),
        // +Z line
        Vec3::new(size, vertical_offset, horizontal_offset),
        Vec3::new(-size, vertical_offset, horizontal_offset),
        // -Z line
        Vec3::new(size, vertical_offset, -horizontal_offset),
        Vec3::new(-size, vertical_offset, -horizontal_offset),
    ]
}

/// Returns the a mesh of vertices for a main grid, along with the grid's size
fn main_grid_vertices_and_size(grid: &Grid, alignment: &GridAlignment) -> (Vec<Vec3>, f32) {
    let size = grid.count as f32 * grid.spacing;
    let vertices = (0..grid.count)
        .map(|offset| (offset + 1) as f32 * grid.spacing)
        .flat_map(|offset| line_vertices(size, offset, 0.0_f32))
        .map(|vertex| alignment.shift_vec3(vertex))
        .collect::<Vec<_>>();
    (vertices, size)
}

/// System for meshing untracked (`Without<TrackedGrid>`) grids
pub fn main_grid_mesher_untracked(
    mut commands: Commands,
    query_parent: Query<
        (Entity, &Grid, Option<&RenderLayers>, Option<&Children>),
        (
            Or<(Changed<Grid>, Changed<RenderLayers>)>,
            Without<TrackedGrid>,
        ),
    >,
    query_children: Query<Entity, With<GridChild>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut simple_materials: ResMut<Assets<SimpleLineMaterial>>,
) {
    for (entity, grid, render_layers, children) in query_parent.iter() {
        let (vertices, _) = main_grid_vertices_and_size(grid, &GridAlignment::default());
        let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::all());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

        if let Some(children) = children {
            despawn_children_of_type(&mut commands, entity, children, &query_children);
        }
        commands.entity(entity).with_children(|children| {
            let mut commands = children.spawn((
                GridChild,
                Mesh3d(meshes.add(mesh)),
                NotShadowCaster,
                Transform::default(),
                Visibility::default(),
                MeshMaterial3d(
                    simple_materials
                        .add(SimpleLineMaterial::from_color(grid.color, grid.alpha_mode)),
                ),
            ));
            if let Some(render_layers) = render_layers {
                commands.insert(render_layers.clone());
            }
        });
    }
}

/// System for meshing tracked (`With<TrackedGrid>`) grids
pub fn main_grid_mesher_tracked(
    mut commands: Commands,
    query_parent: Query<
        (
            Entity,
            &Grid,
            &TrackedGrid,
            Option<&GridAxis>,
            Option<&RenderLayers>,
            Option<&Children>,
        ),
        Or<(
            Changed<Grid>,
            Changed<TrackedGrid>,
            Changed<GridAxis>,
            Changed<RenderLayers>,
        )>,
    >,
    query_children: Query<Entity, With<GridChild>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut clipped_materials: ResMut<Assets<ClippedLineMaterial>>,
) {
    for (entity, grid, tracked, axis, render_layers, children) in query_parent.iter() {
        let (mut vertices, size) = main_grid_vertices_and_size(grid, &tracked.alignment);
        for alignment in [GridAlignment::X, GridAlignment::Z] {
            vertices.extend(
                &GridAxis::create_single_axis(size, alignment)
                    .map(|vertex| tracked.alignment.shift_vec3(vertex)),
            );
        }
        let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::all());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

        if let Some(children) = children {
            despawn_children_of_type(&mut commands, entity, children, &query_children);
        }
        // Main grid lines move with the grid
        commands.entity(entity).with_children(|children| {
            let mut commands = children.spawn((
                GridChild,
                Mesh3d(meshes.add(mesh)),
                NotShadowCaster,
                Transform::default(),
                Visibility::default(),
                MeshMaterial3d(clipped_materials.add(ClippedLineMaterial::new(
                    grid.color,
                    grid.alpha_mode,
                    tracked.alignment,
                    size - grid.spacing,
                    tracked.offset,
                    axis,
                ))),
            ));
            if let Some(render_layers) = render_layers {
                commands.insert(render_layers.clone());
            }
        });

        // Tracked axis line stays at origin
        if let Some(color) = axis.and_then(|axis| axis.get_by_alignment(&tracked.alignment)) {
            let vertices = GridAxis::create_single_axis(size, tracked.alignment).to_vec();
            let mut axis_mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::all());
            axis_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

            let mut axis_entity = commands.spawn((
                GridChild,
                Mesh3d(meshes.add(axis_mesh)),
                NotShadowCaster,
                Transform::default(),
                GlobalTransform::default(),
                Visibility::default(),
                MeshMaterial3d(clipped_materials.add(ClippedLineMaterial::new(
                    color,
                    grid.alpha_mode,
                    tracked.alignment,
                    size - grid.spacing,
                    tracked.offset,
                    None,
                ))),
            ));
            if let Some(render_layers) = render_layers {
                axis_entity.insert(render_layers.clone());
            }
        }
    }
}

/// System for meshing sub-grids
pub fn sub_grid_mesher(
    mut commands: Commands,
    query_parent: Query<
        (
            Entity,
            &Grid,
            &SubGrid,
            Option<&TrackedGrid>,
            Option<&RenderLayers>,
            Option<&Children>,
        ),
        Or<(
            Changed<Grid>,
            Changed<SubGrid>,
            Changed<TrackedGrid>,
            Changed<RenderLayers>,
        )>,
    >,
    query_children: Query<Entity, With<SubGridChild>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut clipped_materials: ResMut<Assets<ClippedLineMaterial>>,
    mut simple_materials: ResMut<Assets<SimpleLineMaterial>>,
) {
    for (entity, grid, sub_grid, tracked, render_layers, children) in query_parent.iter() {
        let size = grid.count as f32 * grid.spacing;
        let sub_spacing = grid.spacing / (sub_grid.count + 1) as f32;

        let alignment = tracked
            .map(|tracked| tracked.alignment)
            .unwrap_or_else(GridAlignment::default);
        let vertices = (0..grid.count)
            .flat_map(|offset| (0..sub_grid.count).map(move |sub_offset| (offset, sub_offset)))
            .map(|(offset, sub_offset)| {
                (sub_offset as f32).mul_add(sub_spacing, offset as f32 * grid.spacing + sub_spacing)
            })
            .flat_map(|offset| line_vertices(size, offset, SUB_GRID_VERTICAL_OFFSET))
            .map(|vertex| alignment.shift_vec3(vertex))
            .collect::<Vec<_>>();
        let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::all());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

        if let Some(children) = children {
            despawn_children_of_type(&mut commands, entity, children, &query_children);
        }
        commands.entity(entity).with_children(|children| {
            let mut child_commands = children.spawn((
                SubGridChild,
                Mesh3d(meshes.add(mesh)),
                NotShadowCaster,
                Transform::from_translation(
                    alignment.shift_vec3(-Vec3::Y * SUB_GRID_VERTICAL_OFFSET),
                ),
                Visibility::default(),
            ));
            if let Some(tracked) = tracked {
                child_commands.insert(MeshMaterial3d(clipped_materials.add(
                    ClippedLineMaterial::new(
                        sub_grid.color,
                        grid.alpha_mode,
                        tracked.alignment,
                        size - grid.spacing,
                        tracked.offset,
                        None,
                    ),
                )));
            } else {
                child_commands.insert(MeshMaterial3d(simple_materials.add(
                    SimpleLineMaterial::from_color(sub_grid.color, grid.alpha_mode),
                )));
            }
            if let Some(render_layers) = render_layers {
                child_commands.insert(render_layers.clone());
            }
        });
    }
}

/// System for meshing grid axis, unless the grid is tracked (`Without<TrackedGrid>`)
pub fn grid_axis_mesher(
    mut commands: Commands,
    query_parent: Query<
        (
            Entity,
            &Grid,
            Option<&GridAxis>,
            Option<&RenderLayers>,
            Option<&Children>,
        ),
        (
            Or<(Changed<Grid>, Changed<GridAxis>, Changed<RenderLayers>)>,
            Without<TrackedGrid>,
        ),
    >,
    query_children: Query<Entity, With<GridAxisChild>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut simple_materials: ResMut<Assets<SimpleLineMaterial>>,
) {
    for (entity, grid, axis, render_layers, children) in query_parent.iter() {
        if let Some(children) = children {
            despawn_children_of_type(&mut commands, entity, children, &query_children);
        }

        commands.entity(entity).with_children(|children| {
            let size = grid.count as f32 * grid.spacing;
            let mut common_axis = Vec::<GridAlignment>::new();
            if let Some(axis) = axis {
                let (used, unused) = axis.create_axis();
                common_axis.extend(&unused);
                for (alignment, color) in used {
                    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::all());
                    mesh.insert_attribute(
                        Mesh::ATTRIBUTE_POSITION,
                        GridAxis::create_single_axis(size, alignment).to_vec(),
                    );
                    let mut commands = children.spawn((
                        GridAxisChild,
                        Mesh3d(meshes.add(mesh)),
                        NotShadowCaster,
                        Transform::default(),
                        Visibility::default(),
                        MeshMaterial3d(
                            simple_materials
                                .add(SimpleLineMaterial::from_color(color, grid.alpha_mode)),
                        ),
                    ));
                    if let Some(render_layers) = render_layers {
                        commands.insert(render_layers.clone());
                    }
                }
            } else {
                common_axis.extend(&GridAxis::default_axis());
            }

            if !common_axis.is_empty() {
                let vertices = common_axis
                    .into_iter()
                    .flat_map(|alignment| GridAxis::create_single_axis(size, alignment))
                    .collect::<Vec<_>>();
                let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::all());
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
                let mut commands = children.spawn((
                    GridAxisChild,
                    Mesh3d(meshes.add(mesh)),
                    NotShadowCaster,
                    Transform::default(),
                    Visibility::default(),
                    MeshMaterial3d(
                        simple_materials
                            .add(SimpleLineMaterial::from_color(grid.color, grid.alpha_mode)),
                    ),
                ));
                if let Some(render_layers) = render_layers {
                    commands.insert(render_layers.clone());
                }
            }
        });
    }
}

/// System which moves tracked grids along with the defined component `T`.
/// Does nothing if the query's `.get_single()` fails.
pub fn tracked_grid_updater<T: Component>(
    mut floor_grid_query: Query<(&mut Transform, &Grid, &TrackedGrid)>,
    tracked_transform_query: Query<&GlobalTransform, (With<T>, Without<TrackedGrid>)>,
) {
    let Ok(tracked_transform) = tracked_transform_query.get_single() else {
        return;
    };
    for (mut grid_transform, grid, tracked) in floor_grid_query.iter_mut() {
        if tracked.tracking_override.is_some() {
            continue;
        };
        let alignment = tracked.alignment.to_inverted_axis_vec3();
        let translation = tracked_transform.translation() * alignment;
        let offset = tracked.alignment.to_axis_vec3() * tracked.offset;
        grid_transform.translation = (translation / grid.spacing).floor() * grid.spacing + offset;
    }
}

/// System which moves tracked grids along with `tracking_override` entity of a `TrackedGrid`.
/// Does nothing if the tracking override entity does not have a `GlobalTransform`.
pub fn custom_tracked_grid_updater(
    mut custom_tracked_grid_query: Query<(&mut Transform, &Grid, &TrackedGrid)>,
    tracked_transform_query: Query<&GlobalTransform>,
) {
    for (mut grid_transform, grid, tracked) in custom_tracked_grid_query.iter_mut() {
        let Some(entity) = tracked.tracking_override else {
            continue;
        };
        let Ok(tracked_transform) = tracked_transform_query.get(entity) else {
            continue;
        };
        let alignment = tracked.alignment.to_inverted_axis_vec3();
        let translation = tracked_transform.translation() * alignment;
        let offset = tracked.alignment.to_axis_vec3() * tracked.offset;
        grid_transform.translation = (translation / grid.spacing).floor() * grid.spacing + offset;
    }
}

/// Despawns children with a marker component upon the removal of their parent
pub fn despawn_children_upon_removal<RemovedParent: Component, ChildMarker: Component>(
    mut removed: RemovedComponents<RemovedParent>,
    query: Query<(&Parent, Entity), With<ChildMarker>>,
    mut commands: Commands,
) {
    if removed.is_empty() {
        return;
    }
    let mut parent_to_child_map: HashMap<Entity, Vec<Entity>> = HashMap::new();
    for (parent, child) in query.iter() {
        parent_to_child_map
            .entry(parent.get())
            .and_modify(|children| children.push(child))
            .or_insert_with(|| vec![child]);
    }
    for entity in removed
        .read()
        .filter_map(|entity| parent_to_child_map.get(&entity))
        .flatten()
    {
        commands.entity(*entity).despawn();
    }
}
