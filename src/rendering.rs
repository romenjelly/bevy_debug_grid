use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::reflect::TypePath;
use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupShaderType, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            ShaderType, SpecializedMeshPipelineError,
        },
    },
};

/// Handle for the clipped line shader
pub const CLIPPED_LINE_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 14735426461149675473);

use crate::{GridAlignment, GridAxis};

/// Material used for tracked grids.
/// It will clip beyond a certain distance from the camera, creating the illusion of an infinite grid.
#[derive(AsBindGroup, TypePath, Debug, Clone, TypeUuid)]
#[uuid = "27cb223e-eb7d-4de3-859f-cb070f13dad3"]
#[uniform(0, ClippedLineMaterialUniform)]
pub struct ClippedLineMaterial {
    pub color: Color,
    pub alpha_mode: AlphaMode,
    pub alignment: GridAlignment,
    pub radius: f32,
    pub offset: f32,
    pub x_axis_color: Color,
    pub y_axis_color: Color,
    pub z_axis_color: Color,
}

impl ClippedLineMaterial {
    pub fn new(
        color: Color,
        alpha_mode: AlphaMode,
        alignment: GridAlignment,
        radius: f32,
        offset: f32,
        axis: Option<&GridAxis>,
    ) -> Self {
        let x_axis_color = axis.and_then(|axis| axis.x).unwrap_or(color);
        let y_axis_color = axis.and_then(|axis| axis.y).unwrap_or(color);
        let z_axis_color = axis.and_then(|axis| axis.z).unwrap_or(color);
        Self {
            color,
            alpha_mode,
            alignment,
            radius,
            offset,
            x_axis_color,
            y_axis_color,
            z_axis_color,
        }
    }
}

/// Uniform for the `ClippedLineMaterial`
#[derive(Clone, Default, ShaderType)]
pub struct ClippedLineMaterialUniform {
    pub color: Color,
    pub alignment: Vec3,
    pub radius: f32,
    pub offset: f32,
    pub x_axis_color: Color,
    pub y_axis_color: Color,
    pub z_axis_color: Color,
}

impl AsBindGroupShaderType<ClippedLineMaterialUniform> for ClippedLineMaterial {
    fn as_bind_group_shader_type(
        &self,
        _images: &RenderAssets<Image>,
    ) -> ClippedLineMaterialUniform {
        ClippedLineMaterialUniform {
            color: self.color,
            alignment: self.alignment.into(),
            radius: self.radius,
            offset: self.offset,
            x_axis_color: self.x_axis_color,
            y_axis_color: self.y_axis_color,
            z_axis_color: self.z_axis_color,
        }
    }
}

impl Material for ClippedLineMaterial {
    fn fragment_shader() -> ShaderRef {
        CLIPPED_LINE_SHADER_HANDLE.typed().into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

/// Handle for the simple line shader
pub const SIMPLE_LINE_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 14181856097290587572);

/// Simple line material with no functionality beyond assigning a color
#[derive(Default, AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "2fbc30f9-03f4-46da-ac0d-de48e7392217"]
pub struct SimpleLineMaterial {
    #[uniform(0)]
    pub color: Color,
    pub alpha_mode: AlphaMode,
}

impl SimpleLineMaterial {
    pub fn new(color: Color, alpha_mode: AlphaMode) -> Self {
        Self { color, alpha_mode }
    }
}

impl Material for SimpleLineMaterial {
    fn fragment_shader() -> ShaderRef {
        SIMPLE_LINE_SHADER_HANDLE.typed().into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}
