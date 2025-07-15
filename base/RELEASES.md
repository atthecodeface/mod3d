# Release 0.3.0 (2025-07-15)

- Remove public access to fields in BufferData which was dangerous

- Changed to using BufferDescriptors, BufferDataAccessors, and BufferIndexAccessors
  to make WebGpu simpler

- Changed from as_slice methods to (generally) AsRef<[u8]> for data/accessors

- Added some more documetation to the Renderable trait, and changed that to use accessors etc

# Release 0.2.0 (2024-12-23)

- Running on Apple Silicon Macs in Sdl2 and Wasm WebGpu

- Partly updated to support buffer descriptors

# Release 0.1.0 (2021-06-22)

- Publishing on crates.io for the first time

- This version is derived from a fairly heavily used private repository, and so is more mature than a normal 0.1.0

- Currently, though, it does not fully support bone poses

**Contributors**: @atthecodeface
