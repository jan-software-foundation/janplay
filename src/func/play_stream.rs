use rodio::Decoder;
use std::io::BufReader;

pub async fn play(file: std::fs::File, sink: &rodio::Sink) {
    let file = BufReader::new(file);
    let source = Decoder::new(file).unwrap();
    //let audio_length = source.total_duration().expect("Audio has unknown length");
    //stream_handle.play_raw(source.convert_samples()).unwrap();
    
    sink.append(source);
}
