use crc::{Crc, CRC_16_IBM_SDLC};
use std::collections::HashMap;
use std::env::current_dir;
use walkdir::WalkDir;

pub const X25: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);

fn main() -> Result<(), std::io::Error> {
    // simple hash map to store <hash as u16, full path to file as string>
    let mut file_map: HashMap<u16, String> = HashMap::new();

    for entry in WalkDir::new(current_dir()?) {
        let entry = entry?;
        let path = entry.path().to_str().unwrap();

        match std::fs::read(path) {
            Ok(bytes) => {
                // TODO: don't read the whole file into memory
                let hash = X25.checksum(&bytes);

                // TODO: only print if verbose is set
                // TODO: print to stderr
                println!("{}: {:?}", path, hash);

                // TODO: check if hash already exists, if it does, stat
                // both files and check if they are really the same file
                // under the assumption that the hash is going to collide
                // often

                file_map.insert(hash, String::from(path));
            }
            Err(_) => {}
        };
    }

    Ok(())
}
