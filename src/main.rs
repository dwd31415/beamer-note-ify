use std::fs;
use std::fmt;
use std::ffi::{OsString};
use std::io::{self, Write};
use std::process::Command;
use strum_macros::EnumString;
use structopt::StructOpt;

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
    /// the LaTeX command
    #[structopt(short = "tex", long = "latex-executable", default_value = "pdflatex")]
    pdflatex: String,
    /// the build folder 
    #[structopt(short = "o", long = "output", default_value = "build", parse(from_os_str))]
    build_folder: std::path::PathBuf,
}

fn build(build_mode : BuildMode, file_name: &std::path::PathBuf, pdflatex: &str, build_folder : &std::path::PathBuf){
    let content = std::fs::read_to_string(file_name)
        .expect("Error: Could not read file.");
    let blocks = parser::parse_input(content).expect("Quit because of a parsing error.");
    let content_maybe = compiler::compile(blocks, build_mode);
    match content_maybe {
        Some(content_final) => {
            let new_path = file_name.with_extension("").with_extension(format!("{}.tex", build_mode)); 
            fs::write(&new_path, content_final).expect("Unable to write the output file.");
            match fs::create_dir_all(build_folder) {
                Ok(()) => {},
                Err(_) => {println!("Could not create output folder. Check permissions.");}
            }
            let mut aux_dir = OsString::from("-aux-directory=");
            aux_dir.push(build_folder.as_os_str());
            let mut out_dir = OsString::from("-output-directory=");
            out_dir.push(build_folder.as_os_str());
            let output = Command::new(pdflatex).
                args(&[aux_dir.as_os_str(), 
                       out_dir.as_os_str(),
                       &new_path.as_os_str()]).output().expect("Could not run LaTeX compiler at the end. Please compile manually.");
            println!("LaTex build status: {}", output.status);
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            if output.status.success() {
                println!("Finished successfully.");
            } else {
                println!("LaTeX compilation failed.");
            }
        },
        None => { println!("There was a fatal error."); }
    }
}

fn main() {
    // Load arguments from the command line
    let args = CliOpt::from_args(); 
    match args.mode {
        BuildMode::Presentation => {build(args.mode, &args.path, args.pdflatex.as_str(), &args.build_folder);},
        BuildMode::LectureNotes => {build(args.mode, &args.path, args.pdflatex.as_str(), &args.build_folder);},
        BuildMode::Both => {build(BuildMode::LectureNotes, &args.path, args.pdflatex.as_str(), &args.build_folder);build(BuildMode::Presentation, &args.path, args.pdflatex.as_str(), &args.build_folder);},
    }
}
