use crc::{Crc, CRC_32_BZIP2};
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::{metadata, File};
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;

fn main() -> Result<(), std::io::Error> {
    // simple hash map to store <hash as u32, full path to file as string>
    let mut file_map: HashMap<u32, String> = HashMap::new();

    for entry in WalkDir::new(current_dir()?) {
        let entry = entry?;
        let path = entry.path().to_str().unwrap();
        let hasher = Crc::<u32>::new(&CRC_32_BZIP2);

        match File::open(path) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut digest = hasher.digest();
                let capacity = reader.capacity();

                loop {

                    let bytes = reader.fill_buf()?;
                    let bytes_read = bytes.len();

                    if bytes_read == 0 || bytes_read <= capacity {
                        break;
                    }

                    digest.update(&bytes);
                }

                let hash = digest.finalize();

                if file_map.contains_key(&hash) {
                    // crc matches, but is this actually the same file?
                    // let's check the file size
                    let existing_path = file_map.get(&hash).unwrap();

                    let this_file = metadata(path)?;
                    let existing_file = metadata(existing_path)?;

                    if this_file.len() == existing_file.len() {
                        println!("{}", path);
                    }
                }

                file_map.insert(hash, String::from(path));
            }
            Err(_) => {}
        }
    }

    Ok(())
}
