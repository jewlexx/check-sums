use sha1::{Digest, Sha1};

use crate::args::{Args, ShaVersion};

mod args;

fn main() {
    let args = Args::parse();

    if args.sha_version == ShaVersion::Sha1 {
        let mut hasher = Sha1::new();

        hasher.update(args.file_contents.as_slice());

        let out = &hasher.finalize()[..];
    }

    println!("Hashing file: {}", args.file_path.display());
}
