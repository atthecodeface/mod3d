# mod3d_gltf

A library that allows loading and saving of Gltf v2.0 files; creation
from within Rust; and creation of mod3d_base models from the Gltf
objects.

It supports gltf with arbitrary uri resolution (so a gltf file that
includes external texture images, for example) and glb format binary
blobs; it also supports base64 encoding of URIs (hence a single file
GLTF object is quite feasible).

Hence it can be used to create a simple GLTF viewer, for example.

## Usage


```
cargo add mod3d_gltf
```

## Features

Serialization and deserialization as optional through the 'serde'
feature; these are required for GLTF and GLB file loading and saving,
but not for creation of mod3d_base objects. Hence the gltf creation API can be used to create mod3d_base models without requiring the serde feature.

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
