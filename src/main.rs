use crc::{Crc, CRC_16_IBM_SDLC};
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::metadata;
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

                if file_map.contains_key(&hash) {
                    // crc matches, but is this actually the same file?
                    // let's check the file size
                    let existing_path = file_map.get(&hash).unwrap();

                    let this_file = metadata(path)?;
                    let existing_file = metadata(existing_path)?;

                    if this_file.len() == existing_file.len() {
                        println!("\n{} == {}: {:?}", path, existing_path, hash);
                    }
                }

                file_map.insert(hash, String::from(path));
            }
            Err(_) => {}
        };
    }

    Ok(())
}
