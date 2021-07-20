use std::{io::Write, process::{Command, Stdio}};
use futures::StreamExt;
use reqwest::Response;

pub async fn transcode(req_response: Response, filepath: String) {
    println!("ffmpeg started");
    let mut command = Command::new("ffmpeg")
        .arg("-i")
        .arg("pipe:0")
        .arg("-f")
        .arg("ogg")
        .arg("-blocksize")
        .arg("2048")
        .arg("-flush_packets")
        .arg("1")
        .arg(format!("{}", filepath))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        //.stderr(Stdio::piped()) 
        .spawn().expect("Failed to summon ffmpeg");
    
    let mut stream = req_response.bytes_stream();
    let mut stdin = command.stdin.take().expect("Failed to open stdin");
    
    while let Some(item) = stream.next().await {
        stdin.write(&item.unwrap()).expect("Failed to write to stdin");
    };
    stdin.flush().expect("Failed to flush stdin");
    println!("ffmpeg done");
}
