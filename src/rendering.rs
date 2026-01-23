use bevy::asset::uuid_handle;
#[allow(unused_imports)]
use bevy::{
    asset::{Asset, Handle},
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    mesh::MeshVertexBufferLayoutRef,
    shader::ShaderRef,
    render::{
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupShaderType, PolygonMode, RenderPipelineDescriptor,
            ShaderType, SpecializedMeshPipelineError,
        },
        texture::GpuImage,
    },
};

/// Handle for the clipped line shader
pub const CLIPPED_LINE_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("66CF2528-BE11-4875-9A37-218FB089E67D");

use crate::{GridAlignment, GridAxis};

/// Material used for tracked grids.
/// It will clip beyond a certain distance from the camera, creating the illusion of an infinite grid.
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
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
    pub color: LinearRgba,
    pub alignment: Vec3,
    pub radius: f32,
    pub offset: f32,
    pub x_axis_color: LinearRgba,
    pub y_axis_color: LinearRgba,
    pub z_axis_color: LinearRgba,
}

impl AsBindGroupShaderType<ClippedLineMaterialUniform> for ClippedLineMaterial {
    fn as_bind_group_shader_type(
        &self,
        _images: &RenderAssets<GpuImage>,
    ) -> ClippedLineMaterialUniform {
        ClippedLineMaterialUniform {
            color: self.color.into(),
            alignment: self.alignment.into(),
            radius: self.radius,
            offset: self.offset,
            x_axis_color: self.x_axis_color.into(),
            y_axis_color: self.y_axis_color.into(),
            z_axis_color: self.z_axis_color.into(),
        }
    }
}

impl Material for ClippedLineMaterial {
    fn fragment_shader() -> ShaderRef {
        CLIPPED_LINE_SHADER_HANDLE.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

/// Handle for the simple line shader
pub const SIMPLE_LINE_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("3E41FD75-3AEA-4B8A-B2CE-6AE5A32973F4");

/// Simple line material with no functionality beyond assigning a color
#[derive(Default, Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct SimpleLineMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    pub alpha_mode: AlphaMode,
}

impl SimpleLineMaterial {
    /// Construct a `SimpleLineMaterial` from a `LinearRgba` and an `AlphaMode`
    pub const fn from_linear_rgba(color: LinearRgba, alpha_mode: AlphaMode) -> Self {
        Self { color, alpha_mode }
    }

    /// Construct a `SimpleLineMaterial` from a `Color` and an `AlphaMode`
    pub fn from_color(color: Color, alpha_mode: AlphaMode) -> Self {
        Self {
            color: color.into(),
            alpha_mode,
        }
    }

    /// Set the color using a `Color` instead of an `LinearRgba`
    pub fn set_color(&mut self, color: Color) {
        self.color = color.into();
    }
}

impl Material for SimpleLineMaterial {
    fn fragment_shader() -> ShaderRef {
        SIMPLE_LINE_SHADER_HANDLE.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}
