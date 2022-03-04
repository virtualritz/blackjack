use crate::prelude::*;

pub mod grid_routine;

/// Some common definitions to abstract wgpu boilerplate
pub mod common;

/// A render routine to draw wireframe meshes
pub mod wireframe_routine;

/// A render routine to draw point clouds
pub mod point_cloud_routine;

/// Shader manager struct which sets up loading with a basic preprocessor
pub mod shader_manager;

/// Adds the necessary nodes to render the 3d viewport of the app. The viewport
/// is rendered into a render target, and its handle is returned.
#[allow(clippy::too_many_arguments)]
pub fn blackjack_viewport_rendergraph<'node>(
    base: &'node r3::BaseRenderGraph,
    graph: &mut r3::RenderGraph<'node>,
    ready: &r3::ReadyData,
    pbr: &'node r3::PbrRoutine,
    tonemapping: &'node r3::TonemappingRoutine,
    grid: &'node grid_routine::GridRoutine,
    wireframe: &'node wireframe_routine::WireframeRoutine,
    point_cloud: &'node point_cloud_routine::PointCloudRoutine,
    resolution: UVec2,
    samples: r3::SampleCount,
    ambient: Vec4,
) -> r3::RenderTargetHandle {
    // Create intermediate storage
    let state = r3::BaseRenderGraphIntermediateState::new(graph, ready, resolution, samples);

    // Preparing and uploading data
    state.pbr_pre_culling(graph);
    state.create_frame_uniforms(graph, base, ambient, resolution);

    // Culling
    state.pbr_shadow_culling(graph, base, pbr);
    state.pbr_culling(graph, base, pbr);

    // Depth-only rendering
    state.pbr_prepass_rendering(graph, pbr, samples);

    // Forward rendering
    state.pbr_forward_rendering(graph, pbr, samples);

    wireframe.add_to_graph(graph, &state);
    point_cloud.add_to_graph(graph, &state);
    grid.add_to_graph(graph, &state);

    // Make the reference to the surface
    let output = graph.add_render_target(r3::RenderTargetDescriptor {
        label: Some("Blackjack Viewport Output".into()),
        resolution,
        samples,
        format: r3::TextureFormat::Bgra8UnormSrgb,
        usage: r3::TextureUsages::RENDER_ATTACHMENT | r3::TextureUsages::TEXTURE_BINDING,
    });
    state.tonemapping(graph, tonemapping, output);

    output
}
