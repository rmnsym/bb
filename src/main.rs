use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "bb",
    author = "Ruman Suyama",
    version = "0.1.0",
    about = "野球の打者指標を算出する"
)]
struct Options {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
    
    #[clap(long, value_name = "JSON", help = "Specify the additional definitions of the build tools.")]
    append_defs: Option<PathBuf>,

    #[clap(short, long, value_name = "JSON", help = "Specify the definition of the build tools.")]
    definition: Option<PathBuf>,

    #[clap(short, long, default_value = "default", value_name = "FORMAT", arg_enum, help = "Specify the output format")]
    format: Format,

    #[clap(short = '@', value_name = "INPUT", help = "Specify the file contains project path list. If INPUT is dash ('-'), read from STDIN.")]
    project_list: Option<String>,

    #[clap(value_name = "PROJECTs", required = false, help = "The target project directories for btmeister.")]
    dirs: Vec<PathBuf>,
}

fn main() {
}
