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
out vec3 View_direction;
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
    View_direction = gl_Position.xyz;
    Normal_frag = (uModelMatrix * uMeshMatrix * vec4(Normal, 0.)).xyz;
    Material_frag = TexCoord;
}
