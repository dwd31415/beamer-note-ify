use structopt::StructOpt;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
enum BuildMode {
    Presentation, 
    LectureNotes,
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
    if args.mode == BuildMode::Both {
        let content = std::fs::read_to_string(&args.path)
            .expect("could not read file");
        for line in content.lines() {
           if line.contains("@bni") {
                println!("{}", line);
            }   
        }
    }
}
