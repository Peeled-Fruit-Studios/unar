use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// The UNix ARchiver, a simple archive utility compatible with MSVC/GNU/LLVM
struct cli {
    /// Format of generated archive
    #[structopt(short = "f", long = "format", default_value = "gnu")]
    format: String,

    /// Output archive name
    #[structopt(short = "o", long = "output", required = true)]
    ofile: String,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let opt = cli::from_args();

    // Check to make sure that the format is one of the following
    // bsd (used by *BSD and darwin), gnu (used by linux and windows) and common (used in .deb packages)
    match opt.format {
        _ if opt.format == "common" => (),
        _ if opt.format == "bsd" => (),
        _ if opt.format == "gnu" => (),
        _ => {
            eprintln!("error: Invalid Format \"{}\"", opt.format);
            process::exit(1);
        }
    }

    println!("{:#?}", opt);
}
