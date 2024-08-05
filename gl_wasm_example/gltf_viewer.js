//a Imports
import init, {CanvasWebgl} from "./pkg/canvas_webgl.js";
// import init, {WasmProject, WasmCip, WasmCameraDatabase, WasmCameraInstance, WasmNamedPoint, WasmNamedPointSet, WasmPointMappingSet, WasmRay} from "../pkg/image_calibrate_wasm.js";
// import {Directory, FileSet} from "./files.js";
// import {Log} from "./log.js";
// import * as html from "./html.js";
// import * as utils from "./utils.js";


const SDP = `{
    "vertex_src": "shaders/vertex.glsl",
    "fragment_src": "shaders/fragment.glsl",
    "attribute_map": {
        "Position":"Position",
        "Normal": "Normal",
	"TexCoord": "TexCoords0"
    }, 
    "uniform_map": {
    "uModelMatrix" : "ModelMatrix",
    "uMeshMatrix": "MeshMatrix",
    "Material" : "Material"
},
    "uniform_buffer_map": {"World": 2},
    "texture_map": {
        "BaseTexture" : ["BaseColor", 0],
	"EmissionTexture": ["Emission", 1],
	"MRTexture": ["MetallicRoughness", 2],
	"OcclusionTexture": ["Occlusion", 3]
    }
    
}
`;
const FRAGMENT = `#version	300 es
precision highp float;

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
    //  Make sure we use Material even though we don't use Material yet
    Color += 0. * Material[0];
    base_color = texture(BaseTexture, Material_frag);
    emission_color = texture(EmissionTexture, Material_frag);
    metallic_roughness = texture(MRTexture, Material_frag);
    occlusion = texture(OcclusionTexture, Material_frag).r * Material[1].y;;
    metallic = metallic_roughness.b * Material[1].x;
    roughness = metallic_roughness.g * Material[1].y;

    view_direction = normalize(View_direction);
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
        if (world.lights[i].position.w < 0.) {
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

	specular_denominator = 4.0 * cos_normal_view * cos_normal_light  + 0.0001;
	specular_light_color_intensity = NDF * ggx1 * ggx2 * specular_fraction * cos_normal_light / specular_denominator;

	// Calculate diffuse fraction and diffuse contribution
	diffuse_fraction = 1.0 - specular_fraction;
	diffuse_fraction = diffuse_fraction * (1.0 - metallic);
	diffuse_light_color_intensity = diffuse_fraction * base_color.rgb * (cos_normal_light / light_distance2) / PI;

	// Sum specular and diffuse contributions, and replace entirely with ambient if ambient
	light_contribution = light_color * (specular_light_color_intensity + diffuse_light_color_intensity);
        if (world.lights[i].position.w == 0.) {
             light_contribution = light_color * base_color.rgb;
	}

	// Add light contribution to pixel
        Color.rgb += light_contribution * occlusion;
    }
    Color += emission_color;
}
`;

const VERTEX = `#version 300 es

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
uniform vec4[2] Material;

void main()
{
    World_position = uModelMatrix * uMeshMatrix * vec4(Position, 1.);
    gl_Position = world.view_matrix * World_position;
    View_direction = gl_Position.xyz;
    Normal_frag = (uModelMatrix * uMeshMatrix * vec4(Normal, 0.)).xyz;
    Material_frag = TexCoord;
}
`;
class Thing {
    constructor(webgl) {
        this.run_step_pending = false;
        this.animating = false;
        this.filename = "ToyCar.glb";
        this.filename = "DamagedHelmet.glb";
        this.node_names = ["0"];
	this.webgl = webgl;
	webgl.add_file("sdp.json", new TextEncoder().encode(SDP));
	webgl.add_file("shaders/fragment.glsl", new TextEncoder().encode(FRAGMENT));
	webgl.add_file("shaders/vertex.glsl", new TextEncoder().encode(VERTEX));
	this.load_glb();
    }
    set_animating(a) {
        console.log("Set animating",a, this.run_step_pending);
        if (a) {
            if (this.run_step_pending) {return;}
            this.animating = true;
            this.init_time = Date.now();
            this.run_step();
        } else {
            this.animating = false;
        }
    }
    run_step() {
        this.run_step_pending = false;
        if (this.animating) {
            this.time_last = this.time;
            this.time = (Date.now() - this.init_time) * 0.001;
            //            this.handle_tick(this.time, this.time_last);
            window.canvas_webgl.fill();
            requestAnimationFrame(()=>this.run_step());
            this.run_step_pending = true;
        }
    }

    //mp fetch_glb
    async fetch_glb() {
        return fetch(this.filename)
            .then((response) => {
                if (!response.ok) {
                    throw new Error(`Failed to fetch interesting points: ${response.status}`);
                }
                return response.blob();
            })
    }

    load_glb() {
        const me = this;
        let promises = [];
        promises.push(
                this.fetch_glb()
                .then((b) => {
                    console.log(b);
                    return b.arrayBuffer();
                    })
                .then((m) => {
                    console.log("Give it buffer", m);
		    this.webgl.add_file(this.filename, m);
                    console.log("Created");
                    })
                    .catch((err) => console.error(`Fetch problem: ${err.message}`))
            );
        Promise.all(promises).then(() => {});
        // ;
    }
    create() {
	this.webgl.create_f('sdp.json', this.filename);
    }
}

//a Top level on load
window.addEventListener("load", (e) => {
    init().then(() => {
        var canvas = document.getElementById('canvas');
        console.log(canvas);
        window.canvas_webgl = new CanvasWebgl(canvas);
        window.thing = new Thing(window.canvas_webgl);
    });
               });
