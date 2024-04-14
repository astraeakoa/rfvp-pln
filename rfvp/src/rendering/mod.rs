//! Everything that is relative to rendering to the window (Like renderable components, camera, transforms..)
use std::ops::Range;

use wgpu::{
    util::BufferInitDescriptor, CommandEncoder, Device, Queue, SurfaceConfiguration, TextureView,
};

use crate::subsystem::world::GameData;
use crate::{config::app_config::AppConfig, subsystem::components::material::Material};

pub(crate) mod gl_representations;
pub(crate) mod renderer_state;
pub(crate) mod shinku2d;
pub(crate) mod shaders;

/// Trait to implement in order to create a renderer to use in the application
pub trait Renderer {
    fn start(&mut self, device: &Device, surface_config: &SurfaceConfiguration);

    /// Will be called first, before render, each time the window request redraw.
    fn update(
        &mut self,
        data: &mut GameData,
        device: &Device,
        surface_config: &SurfaceConfiguration,
        queue: &mut Queue,
    );

    /// Will be called after render, each time the window request redraw.
    fn render(
        &mut self,
        data: &mut GameData,
        config: &AppConfig,
        texture_view: &TextureView,
        encoder: &mut CommandEncoder,
    );
}

pub(crate) trait Renderable2D {
    fn vertex_buffer_descriptor(&mut self, material: Option<&Material>) -> BufferInitDescriptor;
    fn indexes_buffer_descriptor(&self) -> BufferInitDescriptor;
    fn range(&self) -> Range<u32>;
    fn topology() -> wgpu::PrimitiveTopology;
    fn dirty(&self) -> bool;
    fn set_dirty(&mut self, is_dirty: bool);
}

pub(crate) trait RenderableUi: Renderable2D {}