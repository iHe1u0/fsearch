use crate::helper::args_helper::Args;
use crate::helper::file_helper::{find_file, get_file_list};
use clap::Parser;
use std::process::exit;
use std::time::Instant;

mod helper;

fn main() {
    let start = Instant::now();
    let args = Args::parse();
    let all_files = get_file_list(&args.directory).unwrap_or_else(|| {
        println!("File list not found");
        Vec::new()
    });
    if all_files.is_empty() {
        exit(0);
    }
    let found_files = find_file(all_files, &args.filename, Some(false)).unwrap_or_else(|| {
        println!("Not found");
        Vec::new()
    });
    if found_files.is_empty() {
        exit(0);
    }
    dbg!(&found_files);
    if cfg!(debug_assertions) {
        let duration = start.elapsed();
        println!("Total time taken: {:?}", duration);
    }
}
