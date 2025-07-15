# Release 0.3.0 (2025-07-15)

- Remove public access to fields in BufferData which was dangerous

- Changed to using BufferDescriptors, BufferDataAccessors, and BufferIndexAccessors
  to make WebGpu simpler

- Changed from as_slice methods to (generally) AsRef<[u8]> for data/accessors

- Converted the Wasm/WebGPU and OpenGL (+SDL2) to use the newer accessing / Renderable trait

- Made the WGpu crate at least build (which it did not do until now)

# Release 0.0.2 (2024-12-23)

- Running on Apple Silicon Macs in Sdl2 and Wasm WebGpu

- Partly updated to support buffer descriptors

# Release 0.0.1 (2024-08-05)

- Publishing on crates.io for the first time

**Contributors**: @atthecodeface
