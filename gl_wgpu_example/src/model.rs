//a Imports
use mod3d_base::Instance;

use crate::{objects, Model3DWGpu, ShaderInstantiable, ShaderProgram};

//a Light, WorldData
#[derive(Debug, Default)]
#[repr(C)]
pub struct Light {
    position: mod3d_gl::Vec4,
    color: mod3d_gl::Vec4,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct WorldData {
    view_matrix: mod3d_gl::Mat4,
    lights: [Light; 4],
}

//a Base
//tp Base
pub struct Base<'tgt> {
    /// The instantiable objects
    objects: mod3d_base::Instantiable<Model3DWGpu<'tgt>>,
    /// The shader programs
    shader_program: ShaderProgram,
    // Uniform buffers
    // world_gl: UniformBuffer,
}

//tp Instantiable
/// A set of instances that can be instantiated with a particular
/// shader program class
///
/// This has the lifetime of the instantiable data, and is also
/// limited by the lifetime of the shader program class
pub struct Instantiable<'inst, 'prg> {
    /// The set of instances of shader_instantiable (only one of them!)
    instantiables: ShaderInstantiable<'inst, 'prg>,
}

//tp Instances
/// A set of instances that can be drawn in a Model3DWGpu target
///
/// This has the lifetime of the instance, and is also limited by the
/// lifetime of the shader program class
pub struct Instances<'inst, 'tgt> {
    /// The set of instances of objects (only one of them!)
    ///
    /// These are independent of the GL context lifetime
    instance: Instance<'inst, Model3DWGpu<'tgt>>,
}

//ip Base
impl<'tgt> Base<'tgt> {
    //fp new
    pub fn new<'a>(
        wgpu: &'a mut Model3DWGpu<'tgt>,
        shader_program: ShaderProgram,
        filename: &str,
        node_names: &[&str],
    ) -> Result<Self, String> {
        // Use uniform binding point 1 for the material
        //
        // Note that these do not have to match the program's uniform
        // buffer numbering, but they happen to
        let material_uid = 1;
        let world_uid = 2;

        // Bind the program uniform '1' to the uniform binding point 1
        // The base_shader exposes "Material" as program uniform 1
        // let _ = gl.program_bind_uniform_index(&shader_program, 1, material_uid);

        let world_data = [WorldData::default(); 1];
        // let world_gl = gl.uniform_buffer_create(&world_data, true).unwrap();
        // gl.uniform_index_of_range(&world_gl, world_uid, 0, 0);
        // let _ = gl.program_bind_uniform_index(&shader_program, 2, world_uid);

        let objects = objects::new(wgpu, filename, node_names)?;
        Ok(Self {
            objects,
            shader_program,
            // world_gl,
        })
    }

    //fp make_instantiable
    pub fn make_instantiable<'a, 'inst>(
        &'inst self,
        wgpu: &'a mut Model3DWGpu<'tgt>,
    ) -> Result<Instantiable<'inst, 'tgt>, String> {
        let instantiables = ShaderInstantiable::new(wgpu, &self.shader_program, &self.objects)
            .map_err(|_| "Failed to create shader instantiable".to_string())?;
        Ok(Instantiable { instantiables })
    }

    //fp make_instances
    pub fn make_instances<'inst>(&'inst self) -> Instances<'inst, 'tgt> {
        let instance = self.objects.instantiate();
        Instances { instance }
    }

    pub fn update(
        &self,
        wgpu: &Model3DWGpu,
        game_state: &mut GameState,
        instantiable: &Instantiable,
        instances: &mut Instances,
    ) {
        // Update world_gl.gl_buffer world_data[0] (there is only one)
        // view_transformation.rotate_by(&spin);
        // world_data[0].view_matrix = view_transformation.mat4();

        // gl.uniform_buffer_update_data(&self.world_gl, &game_state.world_data, 0);
        // gl.use_program(Some(&self.shader_program));
        // instantiable.instantiables.gl_draw(gl, &instances.instance);

        use geo_nd::quat;
        game_state.spin_axis = quat::apply3(&game_state.axis_spin, &game_state.spin_axis);
        let spin = geo_nd::quat::of_axis_angle(&game_state.spin_axis, 0.01);
        instances.instance.transformation.rotate_by(&spin);
        game_state.time += 0.015;
    }

    //zz All done
}

//a GameState
//tp GameState
pub struct GameState {
    world_data: [WorldData; 1],
    time: f32,
    axis_spin: mod3d_base::Quat,
    spin_axis: mod3d_base::Vec3,
}

//ip GameState
impl GameState {
    pub fn new(scale: f32) -> Self {
        let time: f32 = 0.0;
        let axis_spin = geo_nd::quat::rotate_y(&geo_nd::quat::identity(), 0.01);
        let spin_axis = [1.0, 0.0, 0.0];
        let mut world_data = [WorldData::default(); 1];
        world_data[0].view_matrix[1] = scale;
        world_data[0].view_matrix[4] = scale;
        world_data[0].view_matrix[10] = scale;
        world_data[0].view_matrix[15] = 1.;

        let distant = 0.8;
        let ambient = 0.3;
        world_data[0].lights[0].position = [5., 10., 0., 0.1];
        world_data[0].lights[0].color = [1., 0.4, 0.4, 0.];
        world_data[0].lights[1].position = [-1., 0., 0., 0.1];
        world_data[0].lights[1].color = [0.4, 1., 0.3, 0.];
        world_data[0].lights[2].position = [-1., 0., 0., -1.];
        world_data[0].lights[2].color = [distant, distant, distant, 0.];
        world_data[0].lights[3].position = [0., 0., 0., 0.];
        world_data[0].lights[3].color = [ambient, ambient, ambient, 0.];

        Self {
            world_data,
            time,
            axis_spin,
            spin_axis,
        }
    }
}
