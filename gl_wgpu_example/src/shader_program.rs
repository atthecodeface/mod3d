//a Imports
use std::path::Path;

use crate::Model3DWGpu;

use crate::utils::read_file;
use crate::PipelineDesc;

pub struct ShaderProgram {
    pipeline_desc: PipelineDesc,
    src: String,
}

impl ShaderProgram {
    pub fn create<'tgt>(
        wgpu: &mut Model3DWGpu<'tgt>,
        shader_filename: &str,
        shader_paths: &[&Path],
    ) -> Result<Self, anyhow::Error> {
        let shader_json = read_file(shader_paths, shader_filename)?;
        let pipeline_desc: PipelineDesc = serde_json::from_str(&shader_json)?;

        let frag_src = read_file(shader_paths, pipeline_desc.fragment_src())?; // .map_err(|e| e.to_string()),
        let vert_src = read_file(shader_paths, pipeline_desc.vertex_src())?; //  .map_err(|e| e.to_string()),

        let src = frag_src + &vert_src;
        Ok(Self { pipeline_desc, src })
    }

    //mp compile
    /// Compile the shaders given a vertex buffer layout, index format, and primitive topology
    ///
    /// and uniform groups...
    pub fn compile<'tgt>(
        &self,
        wgpu: &Model3DWGpu<'tgt>,
        vertex_buffer_layout: &[wgpu::VertexBufferLayout],
        index_format: Option<wgpu::IndexFormat>,
        primitive_topology: wgpu::PrimitiveTopology,
        surface_format: wgpu::TextureFormat,
        depth_format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        let shader_module = wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(&self.src)),
        };

        let shader_module = wgpu.device().create_shader_module(shader_module);

        let pipeline_layout_desc = wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[], // &uniform.bind_group_layout],
            push_constant_ranges: &[],
        };

        let pipeline_layout = wgpu.device().create_pipeline_layout(&pipeline_layout_desc);

        let vertex_state = wgpu::VertexState {
            module: &shader_module,
            entry_point: "vertex_main",
            buffers: vertex_buffer_layout,
            compilation_options: Default::default(),
        };

        let primitive_state = wgpu::PrimitiveState {
            topology: primitive_topology,
            strip_index_format: index_format,
            front_face: wgpu::FrontFace::Cw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
            unclipped_depth: false,
        };

        let depth_stencil = wgpu::DepthStencilState {
            format: depth_format,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        };

        let targets = [Some(wgpu::ColorTargetState {
            format: surface_format,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let fragment_state = wgpu::FragmentState {
            module: &shader_module,
            entry_point: "fragment_main",
            targets: &targets,
            compilation_options: Default::default(),
        };

        let pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: vertex_state,
            primitive: primitive_state,
            depth_stencil: Some(depth_stencil),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(fragment_state),
            multiview: None,
            cache: None,
        };

        wgpu.device().create_render_pipeline(&pipeline_descriptor)
    }
}
