//ci VERT_SRC
const VERT_SRC: &str = "
#version 330 core

// Must match ShaderMaterialBaseData in model3d-gltf
struct ShaderMaterialBaseData {
    vec4 base_color;
    vec4 mrxx;
};

struct Light { // 32 bytes
    vec4 position;
    vec4 color;
};

struct WorldData {
    mat4 view_matrix; // 64 bytes
    Light lights[4];  // 128 bytes
};


layout (location = 0) in vec3 Position;
in vec3 Normal;
in vec2 TexCoord;
out vec3 Normal_frag;
out vec4 World_position;
out vec2 Material_frag;


layout(std140) uniform World {
    WorldData world;
};
uniform mat4 uModelMatrix;
uniform mat4 uMeshMatrix;
uniform sampler2D BaseTexture;
// uniform ShaderMaterialBaseData Material;

void main()
{
    World_position = uModelMatrix * uMeshMatrix * vec4(Position, 1.);
    gl_Position = world.view_matrix * World_position;
    Normal_frag = (uModelMatrix * uMeshMatrix * vec4(Normal, 0.)).xyz;
    Material_frag = TexCoord;
}
";

//ci FRAG_SRC
const FRAG_SRC: &str = "
#version	330 core

struct Light { // 32 bytes
    vec4 position;
    vec4 color;
};

struct WorldData {
    mat4 view_matrix; // 64 bytes
    Light lights[4];  // 128 bytes
};

in vec4 World_position;
in vec3 Normal_frag;
in vec2 Material_frag;

out vec4 Color;
uniform sampler2D BaseTexture;
uniform sampler2D EmissionTexture;
uniform vec4[2] Material;

layout(std140) uniform World {
    WorldData world;
};

void main()
{
vec4 base_color;
vec4 emission_color;
    Color = vec4(0.);
    //  Make sure we use Material even though we don't use Material yet
    Color += 0. * Material[0];
    base_color = texture(BaseTexture, Material_frag);
    emission_color = texture(EmissionTexture, Material_frag);
    for(int i=0; i<4; ++i) {
        vec3 light_direction;
        light_direction = world.lights[i].position.xyz - World_position.xyz;
        float distance2;
        distance2 = dot(light_direction, light_direction);
        distance2 = clamp( distance2, world.lights[i].position.w, 1000.);
        if (world.lights[i].position.w <= 0) {
            distance2 = 1.0;
            light_direction = world.lights[i].position.xyz;
        }
        float dot_product = dot( normalize(light_direction), normalize(Normal_frag) );
        dot_product = clamp( dot_product, 0., 1.);
        Color += world.lights[i].color * dot_product / distance2;
    }
 Color *= base_color;
Color += emission_color;
}
";

//fp compile
use model3d_gl::{Gl, GlShaderType};

pub fn compile_shader_program<G: Gl>(model3d: &G) -> Result<<G as Gl>::Program, String> {
    let frag_shader = model3d.compile_shader(GlShaderType::Fragment, FRAG_SRC)?;
    let vert_shader = model3d.compile_shader(GlShaderType::Vertex, VERT_SRC)?;

    model3d.link_program(
        &[&vert_shader, &frag_shader],
        &[
            ("Position", model3d_base::VertexAttr::Position),
            ("Normal", model3d_base::VertexAttr::Normal),
            ("TexCoord", model3d_base::VertexAttr::TexCoords0),
        ],
        &[
            ("uModelMatrix", model3d_gl::UniformId::ModelMatrix),
            ("uMeshMatrix", model3d_gl::UniformId::MeshMatrix),
            ("Material", model3d_gl::UniformId::Material),
        ],
        &[("World", 2)],
        &[
            ("BaseTexture", model3d_gl::TextureId::BaseColor, 0),
            ("EmissionTexture", model3d_gl::TextureId::Emission, 1),
        ],
    )
}
