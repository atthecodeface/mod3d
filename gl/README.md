# mod3d_gl

A library that provides GL object creation, for objects created in
mod3d_base; such objects can then be instantiated arbitrarily at
reasonably high performance.

## Usage


```
cargo add mod3d_gl
```

## Features

A Gpu library backend must be selected; current support is for OpenGL
and WebGL; the former is tested using SDL2 to create OpenGL contexts,
and the latter through Wasm (through the 'mod3d_gl_sdl_example' and
'mod3d_gl_wasm_example' crates.

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
