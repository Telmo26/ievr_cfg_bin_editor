use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

use memmap2::Mmap;

use ievr_cfg_bin_editor_core::parse_database;

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = env::args()
        .nth(1)
        .expect("Usage: ievr_cfg_bin_editor <input_file>");

    let input_path = input_path.trim_matches('"').trim_end_matches("\\"); // This removes trailling backslashes and all quotes
    
    let file_path = PathBuf::from(&input_path);
    let file = File::open(file_path).unwrap();

    let mmap = unsafe { Mmap::map(&file).unwrap() };

    let database = parse_database(&mmap).expect("Failed to parse database");

    let path_buf = PathBuf::from(&input_path);
    let input_file_name = path_buf
        .file_name()
        .expect("Invalid input file")
        .to_string_lossy();

    let output_path = PathBuf::from(format!("{input_file_name}.json"));

    let mut file = File::create(output_path).unwrap();
    file.write_all(database.serialize().as_bytes())?;

    Ok(())
}
