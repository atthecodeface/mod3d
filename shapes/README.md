# mod3d_shapes

This will provide some simple shapes to allow creation of 3d objects for mod3d_base and mod3d_gltf.

As such the models will be able to be rendered for simple games
through the GPU implementations in mod3d_gl, and also to be saved as
GLTF files as assets to be loaded in other systems, or games using the
mod3d infrastructure.

Currently it supports a (subdividable) icosphere with texture mapping
to provide high definition spherical image compression, and gridded
rectangular regions that will permit extrusions to produce 3D objects.

## Usage


```
cargo add mod3d_shapes
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
