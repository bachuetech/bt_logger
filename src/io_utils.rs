use std::{fs::OpenOptions, io::{BufWriter, Write}};

pub fn log_to_file(file: &str, message: &str) {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file)
        .unwrap();
    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}", message).unwrap();
    writer.flush().unwrap(); // This forces the write
}