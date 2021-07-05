use rodio::{Decoder, OutputStream};
use std::io::{BufReader, Read};
use std::fs::File;

pub async fn play(stream: std::process::ChildStdout) {
    let path = ulid::Ulid::new().to_string();
    let mut buf_reader = BufReader::new(stream);
    
    if !std::path::Path::new(&path).exists() {
        std::fs::write(&path, "").unwrap();
    }
    
    
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(&path).unwrap());
    let source = Decoder::new(file).unwrap();
}