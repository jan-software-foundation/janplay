use rodio::{Decoder, OutputStream, Source};
use std::io::BufReader;
use std::thread;

pub async fn play(file: std::fs::File) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(file);
    let source = Decoder::new(file).unwrap();
    let  audio_length = source.total_duration().expect("Audio has unknown length");
    stream_handle.play_raw(source.convert_samples()).unwrap();
    
    thread::sleep(audio_length);
}
