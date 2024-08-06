//a Imports
use mod3d_base::Instance;
use mod3d_gl::{Gl, ShaderInstantiable, UniformBuffer};

use crate::objects;

//a Light, WorldData
#[derive(Debug, Default)]
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
pub struct Base<G: Gl> {
    /// The instantiable objects
    objects: mod3d_base::Instantiable<G>,
    /// The shader programs
    shader_program: G::Program,
    /// Uniform buffers
    world_gl: UniformBuffer<G>,
}

//tp Instantiable
/// Borrows from Base
pub struct Instantiable<'inst, G: Gl> {
    /// The set of instances of shader_instantiable (only one of them!)
    instantiables: mod3d_gl::ShaderInstantiable<'inst, G>,
}

//tp Instances
/// Borrows from Base
pub struct Instances<'inst, G: Gl> {
    /// The set of instances of objects (only one of them!)
    ///
    /// These are independent of the GL context lifetime
    instance: Instance<'inst, G>,
}

//ip Base
impl<G: Gl> Base<G> {
    //fp new
    pub fn new(
        gl: &mut G,
        shader_program: G::Program,
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
        let _ = gl.program_bind_uniform_index(&shader_program, 1, material_uid);

        let world_data = [WorldData::default(); 1];
        let world_gl = gl.uniform_buffer_create(&world_data, true).unwrap();
        gl.uniform_index_of_range(&world_gl, world_uid, 0, 0);
        let _ = gl.program_bind_uniform_index(&shader_program, 2, world_uid);

        let objects = objects::new(gl, filename, node_names)?;
        Ok(Self {
            objects,
            shader_program,
            world_gl,
        })
    }

    //fp make_instantiable
    pub fn make_instantiable<'inst>(
        &'inst self,
        gl: &mut G,
    ) -> Result<Instantiable<'inst, G>, String> {
        let instantiables = ShaderInstantiable::new(gl, &self.shader_program, &self.objects)
            .map_err(|_| "Failed to create shader instantiable".to_string())?;
        Ok(Instantiable::<G> { instantiables })
    }

    //fp make_instances
    pub fn make_instances(&self) -> Instances<'_, G> {
        let instance = self.objects.instantiate();
        Instances { instance }
    }

    pub fn update(
        &self,
        gl: &mut G,
        game_state: &mut GameState,
        instantiable: &Instantiable<G>,
        instances: &mut Instances<G>,
    ) {
        // Update world_gl.gl_buffer world_data[0] (there is only one)
        // view_transformation.rotate_by(&spin);
        // world_data[0].view_matrix = view_transformation.mat4();

        gl.uniform_buffer_update_data(&self.world_gl, &game_state.world_data, 0);
        gl.use_program(Some(&self.shader_program));
        instantiable.instantiables.gl_draw(gl, &instances.instance);

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
