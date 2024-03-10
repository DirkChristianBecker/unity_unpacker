use std::path::{Path, PathBuf};

use unity_unpacker_lib::{UnityPackage, UnityPackageReaderError};
use clap::Parser;
use rust_tools::prelude::Console;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file_name: String,

    /// Number of times to greet
    #[arg(short, long)]
    target_dir: Option<String>,
}

fn main() {
    let args = Args::parse();
    let package = UnityPackage::new(&args.file_name).unwrap();
    let r : Result<PathBuf, UnityPackageReaderError>;
    if let Some(s) = args.target_dir {
        let p = &Path::new(&s);
        r = package.unpack_package(Some(p));        
    } else {
        r = package.unpack_package(None);
    }

    match r {
        Ok(e) => {
            print!("{} extracted to {}", args.file_name, e.into_os_string().into_string().unwrap());
        },
        Err(e) => {
            // eprint!("\x1b[93mCould not extract {}. Error: {}", args.file_name, e);
            eprintln!("{}error: {}Could not extract the file '{}{}{}'. {}{}.", 
                Console::FAIL, 
                Console::RESET, 
                Console::OK_BLUE, 
                args.file_name,
                Console::RESET,
                Console::RED,
                e);
        },
    }
}
