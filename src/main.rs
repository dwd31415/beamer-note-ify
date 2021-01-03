use structopt::StructOpt;
use strum_macros::EnumString;

mod parser;
mod compiler;
mod data_structures;
mod preprocessor;

#[derive(Copy, Clone, Debug, PartialEq, EnumString)]
pub enum BuildMode {
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
    let content = std::fs::read_to_string(&args.path)
        .expect("Error: Could not read file.");
    let blocks = parser::parse_input(content).expect("Quit because of a parsing error.");
    println!("{}",compiler::compile(blocks, args.mode).unwrap());
}
