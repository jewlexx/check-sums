use pico_args::Arguments;

pub enum ShaVersion {
    Sha1,
    Sha256,
    Sha512,
}

pub struct Args {
    pub sha_version: ShaVersion,
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

        Args { sha_version }
    }
}
