use std::path::PathBuf;

use sha1::{Digest, Sha1};

use crate::args::{Args, ShaVersion};

mod args;

fn main() {
    let args = Args::parse();

    // if args.sha_version == ShaVersion::Sha1 {
    //     let hasher = Sha1::new();
    // }

    println!("Hashing file: {}", args.file_path.display());
}
