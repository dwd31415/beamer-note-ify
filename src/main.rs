use structopt::StructOpt;
use std::fs;
use strum_macros::EnumString;
use std::fmt;

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

impl fmt::Display for BuildMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(StructOpt)]
struct CliOpt {
    /// the build mode
    mode: BuildMode, 
    /// the file name of the TeX main file
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn build(build_mode : BuildMode, file_name: &std::path::PathBuf){
    let content = std::fs::read_to_string(file_name)
        .expect("Error: Could not read file.");
    let blocks = parser::parse_input(content).expect("Quit because of a parsing error.");
    let content_maybe = compiler::compile(blocks, build_mode);
    match content_maybe {
        Some(content_final) => {
            let new_path = file_name.with_extension("").with_extension(format!("{}.tex", build_mode)); 
            fs::write(new_path, content_final).expect("Unable to write the output file.");
        },
        None => { println!("There was a fatal error."); }
    }
}

fn main() {
    // Load arguments from the command line
    let args = CliOpt::from_args(); 
    match args.mode {
        BuildMode::Presentation => {build(args.mode, &args.path);},
        BuildMode::LectureNotes => {build(args.mode, &args.path);},
        BuildMode::Both => {build(BuildMode::LectureNotes, &args.path);build(BuildMode::Presentation, &args.path);},
    }
}
