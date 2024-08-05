.PHONY: test_builds
test_builds:
	cargo build

.PHONY: test
test: test_tests

.PHONY: test_tests
test_tests:
	cargo test

.PHONY: features
features:
	cargo tree -e features


PHONY: sdl_build
sdl_build:
	cargo build -p mod3d-gl-sdl-example --release

sdl_view: sdl_build
	./target/release/mod3d-gl-sdl-example --shader shaders/sdp.json --glb glb/DamagedHelmet.glb --scale 0.5

test_builds: test_build_base
test_build_base:
	cargo build -p mod3d-base --no-default-features --features mod3d-base/serde

test_builds: test_build_gl_serde
test_build_gl_serde:
	cargo build -p mod3d-gl --no-default-features --features mod3d-gl/serde

test_builds: test_build_gl_webgl
test_build_gl_webgl:
	cargo build -p mod3d-gl --no-default-features --features mod3d-gl/webgl

test_builds: test_build_gl_opengl
test_build_gl_opengl:
	cargo build -p mod3d-gl --no-default-features --features mod3d-gl/opengl

test_builds: test_build_gltf_serde
test_build_gltf_serde:
	cargo build -p mod3d-gltf --no-default-features --features mod3d-gltf/serde

test_builds: test_build_gltf_serde_json
test_build_gltf_serde_json:
	cargo build -p mod3d-gltf --no-default-features --features mod3d-gltf/serde_json

test_builds: test_build_shapes
test_build_shapes:
	cargo build -p mod3d-shapes





test_tests: test_test_base
test_test_base:
	cargo test -p mod3d-base --no-default-features --features mod3d-base/serde

test_tests: test_test_gl_serde
test_test_gl_serde:
	cargo test -p mod3d-gl --no-default-features --features mod3d-gl/serde

test_tests: test_test_gl_webgl
test_test_gl_webgl:
	cargo test -p mod3d-gl --no-default-features --features mod3d-gl/webgl

test_tests: test_test_gl_opengl
test_test_gl_opengl:
	cargo test -p mod3d-gl --no-default-features --features mod3d-gl/opengl

test_tests: test_test_gltf_serde
test_test_gltf_serde:
	cargo test -p mod3d-gltf --no-default-features --features mod3d-gltf/serde

test_tests: test_test_gltf_serde_json
test_test_gltf_serde_json:
	cargo test -p mod3d-gltf --no-default-features --features mod3d-gltf/serde_json


