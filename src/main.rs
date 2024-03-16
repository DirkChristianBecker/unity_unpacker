use unity_unpacker_lib::prelude::{UnityPackage, UnityPackageReaderError};
use clap::Parser;
use rust_tools::prelude::Console;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the .unitypackage file.
    #[arg(short, long)]
    file_name: String,

    /// The path to extract to
    #[arg(short, long)]
    dir: Option<String>,

    /// A tmp path to extract the package temporarilly
    #[arg(short, long)]
    tmp_dir: Option<String>,

    /// Remove the tmp directory after the package has been extracted?
    #[arg(short, long, default_value = "true")]
    remove_tmp: Option<bool>,
}

fn main() {
    let args = Args::parse();
    let mut package = UnityPackage::new(&args.file_name, args.dir, args.tmp_dir).unwrap();
    let mut delete_tmp = true;
    if let Some(p) = args.remove_tmp { delete_tmp = p; }

    let r : Result<(), UnityPackageReaderError> = package.unpack_package(delete_tmp);

    let target = match package.get_target_dir() {
        Ok(e) => { e }
        Err(_) => panic!("")
    };

    match r {
        Ok(()) => {
            print!("{}ok: {}'{}' extracted to '{:?}'", Console::OK_GREEN, Console::RESET, args.file_name, target);
        },
        Err(e) => {
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
