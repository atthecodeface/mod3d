// ./target/release/mod3d-gl-sdl-example --shader sdp.json --node 0 --scale 0.1 --glb MetalRoughSpheres.glb
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};

use mod3d_gl::Model3DOpenGL;

mod model;
mod objects;
mod sdl_window;
mod utils;

//fp add_shader_arg
pub fn add_shader_arg(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("shader")
            .long("shader")
            .short('s')
            .required(true)
            .help("Shader program descriptor")
            .action(ArgAction::Set),
    )
}

pub fn add_glb_arg(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("glb")
            .long("glb")
            .short('g')
            .required(true)
            .help("GLB file to read")
            .action(ArgAction::Set),
    )
}

pub fn add_node_arg(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("node")
            .long("node")
            .short('n')
            .help("Node to view")
            .action(ArgAction::Append),
    )
}

pub fn add_scale_arg(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("scale")
            .long("scale")
            .short('S')
            .default_value("1")
            .help("Scale factor to apply to object")
            .value_parser(value_parser!(f32))
            .action(ArgAction::Set),
    )
}

fn load_shader_program(matches: &ArgMatches) -> Result<mod3d_gl::ShaderProgramDesc, anyhow::Error> {
    let shader_filename = matches.get_one::<String>("shader").unwrap();
    let shader = std::fs::read_to_string(shader_filename)?;
    Ok(serde_json::from_str(&shader)?)
}
fn main() -> Result<(), anyhow::Error> {
    let cmd = Command::new("gltf_viewer")
        .about("Gltf viewer")
        .version("0.1.0");
    let cmd = add_shader_arg(cmd);
    let cmd = add_glb_arg(cmd);
    let cmd = add_node_arg(cmd);
    let cmd = add_scale_arg(cmd);
    let matches = cmd.get_matches();

    let scale = *matches.get_one::<f32>("scale").unwrap();

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

    let shader_program_desc = load_shader_program(&matches)?;

    let mut sdl_window = sdl_window::SdlWindow::new()?;

    let mut model3d = Model3DOpenGL::new();
    let base = model::Base::new(
        &mut model3d,
        &[&std::path::Path::new("../")],
        &shader_program_desc,
        glb_filename,
        &node_name_refs,
    )
    .unwrap();
    let instantiables = base.make_instantiable(&mut model3d).unwrap();
    let mut game_state = model::GameState::new(scale);
    let mut instances = base.make_instances();

    sdl_window.prepare_viewport();

    sdl_window.prepare_event_loop();
    // main loop
    'main: loop {
        for event in sdl_window.event_poll() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    // Don't need to do this - it is automatic
                    //
                    // But the drawable is NOT the window size it is the window size
                    // modified by Retinaness
                    //                    let (w, h) = window.drawable_size();
                    unsafe { gl::Viewport(0, 0, w, h) };
                }
                _ => {}
            }
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
