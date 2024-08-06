use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};

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

pub fn scale(matches: &ArgMatches) -> f32 {
    *matches.get_one::<f32>("scale").unwrap()
}
pub fn shader(matches: &ArgMatches) -> String {
    matches.get_one::<String>("shader").unwrap().to_owned()
}
