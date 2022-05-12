use std::path::PathBuf;

use pico_args::Arguments;

#[derive(PartialEq, Eq)]
pub enum ShaVersion {
    Sha1,
    Sha256,
    Sha512,
}

impl ShaVersion {}

pub struct Args {
    pub sha_version: ShaVersion,
    pub file_path: PathBuf,
}

impl Args {
    pub fn parse() -> Self {
        let mut args = Arguments::from_env();

        let sha1 = args.contains("--sha1");
        let sha256 = args.contains("--sha256");
        let sha512 = args.contains("--sha512");

        let sha_version = if sha1 {
            ShaVersion::Sha1
        } else if sha256 {
            ShaVersion::Sha256
        } else if sha512 {
            ShaVersion::Sha512
        } else {
            println!("WARNING: using default SHA-1");
            ShaVersion::Sha1
        };

        let path = args.free_from_fn(|p| {
            let cwd = std::env::current_dir().unwrap();
            let path = cwd.join(p);

            if path.exists() {
                if path.is_file() {
                    Ok(path)
                } else {
                    Err("path is not a file")
                }
            } else {
                Err("file does not exist")
            }
        });

        if let Some(e) = path.clone().err() {
            println!("{}", e);
            std::process::exit(1);
        }

        Args {
            sha_version,
            file_path: path.unwrap(),
        }
    }
}
