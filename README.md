# mod3d

This provides a set of libraries for 3D model object generation,
import/export as GLTF, and rendering in various GPU front-ends at
reasonably high performance.

It also provides some example uses:

* An SDL2 Gltf viewer

* A Wasm WebGl Gltf viewer, which runs in a browser

* An as-yet unfinished Wgpu gltf viewer

# Sample Glb files

For testing the Gltf viewers, some Glb files are generally used. Examples are:

* A rubber duck: https://github.com/KhronosGroup/glTF-Sample-Models/blob/main/2.0/Duck/glTF-Binary/Duck.glb

* A damaged helmet: https://github.com/KhronosGroup/glTF-Sample-Models/tree/main/2.0/DamagedHelmet/glTF-Binary/DamagedHelmet.glb

* A water bottle: https://github.com/KhronosGroup/glTF-Sample-Models/blob/main/2.0/WaterBottle/glTF-Binary/WaterBottle.glb

It is common to keep these locally in a 'glb' directory - the web
viewer particularly expects to find them there.

# Sdl2 GltfViewer

A simple Gltf viewer that uses Sdl2 as its window interface, with
OpenGL as the graphics library.

## Building

The binary of the Gltf viewer for SDL can be build locally with:

```
cargo build --release -p mod3d-gl-sdl-example
```

## Invocation

The SDL gltf viewer is the default binary for mod3d-gl-sdl-example.

```
../target/release/mod3d-gl-sdl-example --help
```

It provides a viewer that will open a window and, using a spinning
camera, attempt to display the contents of a Glb file (binary
Gltf). It uses shaders that are provided on the command line, with a
Json file that describes the shader layout and the shader sources.

```
../target/release/mod3d-gl-sdl-example --shader ../shaders/sdp.json --glb ../glb/DamagedHelmet.glb
```

# Wasm WebGl GltfViewer

A simple Gltf viewer that runs in a browser and uses WebGl as the graphics library.

## Building

The package for the web can be built with:

```
(cd gl_wasm_example && ./mk)
```

## Invocation

A web browser is required to view Gltf files; it should be pointed at
an HTTP server with root directory in this crate source. This can be run with:

```
python3 -m http.server 3000
```

The browse to http://127.0.0.1:3000/gl_wasm_example/

The index.html loads the gltf_viewer.js, which in turn loads the Wasm-compiled rust code.

The gltf_viewer.js file currently has the shader description Json file
and the Glb file to load hard-coded.

# Shaders for Sdl2 OpenGL and WebGl

The same shaders are used for OpenGL and WebGL.

## Shader Json

The shader Json provides a dictionary of:

* vertex_src - the filename of the vertex Glsl shader source

* fragment_src - the filename of the fragment Glsl shader source

* attribute_map - a dictionary of mapping of *shader* attribute names to Gltf shader attributes

* uniform_map - a dictionary of mapping of *shader* uniform names to
  ModelMatrix, MeshMatrix, and Material - these are viewer-internal
  uniforms that describe the position of a model, and the material to
  draw a mesh in

* uniform_buffer_map - a map from shader uniform buffer names to ?

* texture_map - a map from shader texture (sampler2D) buffer names to Gltf texture object names

## Shader data structures

The gltf viewer uses a WorldData uniform buffer that contains a mat4
of the view matrix, and then an array of 4 Lights - each of which is a
vec4 position, vec4 color.

## Shader vertex data

Each shader vertex is provided with:

* its mesh-relative position; this should have the MeshMatrix applied
  to it, and then the ModelMatrix, to yield a World position.

* its mesh-relative normal; this should have the MeshMatrix applied
  to it, and then the ModelMatrix, to yield a World normal.

* its Vec2D texture coordinate (used for all textures it is part of)

The vertex shader is supplied with the WorldData uniform that includes
the mat4 view matrix, which can be applied to the world position to
generate a view-relative position.

## Shader fragment data

The fragment shader has access to the WorldData uniform, which
supplies the locations of the lights.

It will also have the texture samplers set up for the textures
declared in its Json (the Json maps these fragment shader uniform
names to Gltf-named texture samplers).

## Usage

```
cargo add mod3d
```

An install can be performed with:

```
cargo install mod3d-gl-sdl-example
```

## Releases

Release notes are available in [RELEASES.md](RELEASES.md).

## License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
