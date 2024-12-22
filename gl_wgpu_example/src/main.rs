// ./target/release/mod3d-gl-sdl-example --shader sdp.json --node 0 --scale 0.1 --glb MetalRoughSpheres.glb
use clap::Command;

mod wgpu;
// use mod3d_gl::Model3DOpenGL;

mod cmdline;
mod model;
mod objects;
// mod shader_program;
mod types;
mod utils;
mod wgpu_window;

fn main() -> Result<(), anyhow::Error> {
    let cmd = Command::new("gltf_viewer")
        .about("Gltf viewer")
        .version("0.1.0");

    let cmd = cmdline::add_shader_arg(cmd);
    let cmd = cmdline::add_glb_arg(cmd);
    let cmd = cmdline::add_node_arg(cmd);
    let cmd = cmdline::add_scale_arg(cmd);

    let matches = cmd.get_matches();

    let scale = cmdline::scale(&matches);
    let shader_filename = cmdline::shader(&matches);

    let glb_filename = matches.get_one::<String>("glb").unwrap();
    let mut node_names = vec![];
    if let Some(values) = matches.get_many::<String>("node") {
        for v in values {
            node_names.push(v.to_string());
        }
    } else {
        node_names.push("0".to_string());
    }
    let node_name_refs: Vec<&str> = node_names.iter().map(|s| &**s).collect();

    let mut sdl_window = wgpu_window::WGpuWindow::new()?;
    let mut model3d = wgpu::Model3DWGpu::new(sdl_window.window());

    let shader_paths = [std::path::Path::new("../")];

    let shader_program =
        shader_program::create_shader_program(&mut model3d, &shader_filename, &shader_paths)?;

    let base =
        model::Base::new(&mut model3d, shader_program, glb_filename, &node_name_refs).unwrap();
    let instantiables = base.make_instantiable(&mut model3d).unwrap();
    let mut game_state = model::GameState::new(scale);
    let mut instances = base.make_instances();

    sdl_window.prepare_viewport();

    // main loop
    'main: loop {
        match sdl_window.event_poll() {
            types::Event::Quit => break 'main,
            types::Event::ResizeWindow(w, h) => {
                sdl_window.resize_viewport(0, 0, w, h);
                continue 'main;
            }
            _ => (),
        }

        sdl_window.clear_framebuffer();

        base.update(
            &mut model3d,
            &mut game_state,
            &instantiables,
            &mut instances,
        );

        sdl_window.swap_framebuffer();

        let ten_millis = std::time::Duration::from_millis(10);
        // let pre = std::time::Instant::now();
        std::thread::sleep(ten_millis);
        // let post = std::time::Instant::now();
        // dbg!(post - pre);
    }
    Ok(())
}
