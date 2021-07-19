use std::{io::Write, process::{Command, Stdio}};
use futures::StreamExt;
use reqwest::Response;

pub async fn transcode(req_response: Response, filename: String) {
    println!("ffmpeg started");
    let mut command = Command::new("sh")
        .arg("-c")
        .arg(format!("ffmpeg -i pipe:0 -f wav \"{}\"", filename))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped()) 
        .spawn().expect("Failed to summon ffmpeg");
    
    let mut stream = req_response.bytes_stream();
    let mut stdin = command.stdin.take().expect("Failed to open stdin");
    
    while let Some(item) = stream.next().await {
        stdin.write(&item.unwrap()).expect("Failed to write to stdin");
    };
    stdin.flush().expect("Failed to flush stdin");
    println!("ffmpeg done");
}
