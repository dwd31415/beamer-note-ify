use structopt::StructOpt;
use strum_macros::EnumString;

mod parser;

#[derive(Debug, PartialEq, EnumString)]
enum BuildMode {
    #[strum(serialize = "presentation", serialize = "p")]
    Presentation, 
    #[strum(serialize = "paper", serialize = "lecture-notes", serialize="notes", serialize="ln", serialize="n")]
    LectureNotes,
    #[strum(serialize = "both", serialize = "b")]
    Both
}

#[derive(StructOpt)]
struct CliOpt {
    /// the build mode
    mode: BuildMode, 
    /// the file name of the TeX main file
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    // Load arguments from the command line
    let args = CliOpt::from_args(); 
    if args.mode == BuildMode::Presentation {
        let content = std::fs::read_to_string(&args.path)
            .expect("could not read file");
        parser::parse_input(content);
    }
}
