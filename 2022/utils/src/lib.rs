use std::env;
use std::fs::File;
use std::io::BufReader;

pub fn get_file_reader(file_path: &str) -> BufReader<File> {
    let curr_dir = env::current_dir().unwrap().display().to_string();
    let path_to_input = format!("{}/{}", curr_dir, file_path);

    let file = File::open(path_to_input).unwrap();
    let reader = BufReader::new(file);

    reader
}