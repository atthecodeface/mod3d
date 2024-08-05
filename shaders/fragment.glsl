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
in vec3 View_direction;
in vec3 Normal_frag;
in vec2 Material_frag;

out vec4 Color;
uniform sampler2D BaseTexture;
uniform sampler2D EmissionTexture;
uniform sampler2D MRTexture;
uniform sampler2D OcclusionTexture;
uniform vec4[2] Material;

layout(std140) uniform World {
    WorldData world;
};


float DistributionGGX(float NdotH, float roughness)
{
    float a      = roughness*roughness;
    float a2     = a*a;
    float NdotH2 = NdotH*NdotH;
	
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = 3.14159265 * denom * denom;
	
    return a2 / denom;
}

// The standard Fresnel-Schlick equation,
//
// F0 is a grey-scale (up to 0.17 for diamond) for nonmetallic
// surfaces - generally around 0.04
//
// F0 is the material color for metallic surfaces (where the material
// color is dulled for iron, for example, for which F0 is (0.56, 0.56,
// 0.56), but for gold is (1.0, 0.71, 0.29) )
//
// For a patch on a surface that is 30% metal and 70% nonmetal
// (patches are non-zero area) then one can fake this out (ditching
// diamond!) with 70% of (0.04,0.04,0.04) and 30% of material color
vec3 Fresnel(vec3 h, vec3 v, vec3 color, float metallic)
{
  float cos_theta = max(dot(h, v), 0.);
    vec3 F0 = mix( vec3(0.04), color, metallic);
    return F0 + (1.0 - F0) * pow(1.0 - cos_theta, 5.0);
}

void main()
{
    const float PI = 3.14159265359;
  
    vec4 base_color;
    vec4 emission_color;
    vec4 metallic_roughness;
    float metallic;
    float roughness;
    float occlusion;
    vec3 view_direction;
    vec3 normal;
    
    Color = vec4(0.);
    base_color = texture(BaseTexture, Material_frag);
    emission_color = texture(EmissionTexture, Material_frag);
    metallic_roughness = texture(MRTexture, Material_frag);
    occlusion = texture(OcclusionTexture, Material_frag).r; // * Material[1].z;
    metallic = metallic_roughness.b * Material[1].x;
    roughness = metallic_roughness.g * Material[1].y;

    view_direction = normalize(-View_direction);
    normal = normalize(Normal_frag);

    float cos_normal_view;
    cos_normal_view = max(dot(normal, view_direction), 0.0);
    
    float r_plus_1 = (roughness + 1.0);
    float schlick_ggx_k = (r_plus_1 * r_plus_1) / 8.0;
    float roughness2      = roughness*roughness;
    float roughness4      = roughness2*roughness2;
    float ggx1  = cos_normal_view / (cos_normal_view * (1.0 - schlick_ggx_k) + schlick_ggx_k);
    
    for(int i=0; i<4; ++i) {
        vec3 light_direction;
        vec3 light_color;
	float cos_normal_light;
        float light_distance2;

	vec3 halfway_vector;
	float cos_normal_halfway;
	float cos_normal_halfway2;

	float ggx2;
	float NDF;
	float NDF_denom;
	vec3 specular_fraction;
	float specular_denominator;
	vec3 specular_light_color_intensity;

	vec3 diffuse_fraction;
	vec3 diffuse_light_color_intensity;

	vec3 light_contribution;

	// Calculate light direction and fall-off
        light_direction = world.lights[i].position.xyz - World_position.xyz;
        light_color = world.lights[i].color.rgb;
	
        light_distance2 = dot(light_direction, light_direction);
        light_distance2 = clamp( light_distance2, world.lights[i].position.w, 1000.);
        if (world.lights[i].position.w < 0) {
            light_distance2 = 0.5;
            light_direction = world.lights[i].position.xyz;
        }
	light_direction = normalize(light_direction);

	cos_normal_light = max(dot( light_direction, normal ), 0.);

	// Calculate halfway_vector
 	halfway_vector = normalize(light_direction + view_direction);
	cos_normal_halfway = max(dot(halfway_vector, normal),0.);
	cos_normal_halfway2 = cos_normal_halfway*cos_normal_halfway;

	// Fresnel-Schlick calculation to determine specular fraction
	specular_fraction = Fresnel(halfway_vector, view_direction, base_color.rgb, metallic);

	// Distribution and Schlick thing for specular contribution
	ggx2 = cos_normal_light / (cos_normal_light * (1.0 - schlick_ggx_k) + schlick_ggx_k);

	NDF_denom = 1.0 + cos_normal_halfway2 * (roughness4 - 1.0);
	NDF = roughness4 / (PI * NDF_denom * NDF_denom);

	specular_denominator = 4 * cos_normal_view * cos_normal_light  + 0.0001;
	specular_light_color_intensity = NDF * ggx1 * ggx2 * specular_fraction * cos_normal_light / specular_denominator;

	// Calculate diffuse fraction and diffuse contribution
	diffuse_fraction = 1.0 - specular_fraction;
	diffuse_fraction = diffuse_fraction * (1.0 - metallic);
	diffuse_light_color_intensity = diffuse_fraction * base_color.rgb * (cos_normal_light / light_distance2) / PI;

	// Sum specular and diffuse contributions, and replace entirely with ambient if ambient
	light_contribution = light_color * (specular_light_color_intensity + diffuse_light_color_intensity);
        if (world.lights[i].position.w == 0) {
             light_contribution = light_color * base_color.rgb;
	}

	// Add light contribution to pixel
        Color.rgb += light_contribution * occlusion;
    }
    Color += 1.0 * emission_color;
}
