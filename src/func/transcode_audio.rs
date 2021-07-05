use std::{io::Write, process::{Command, Stdio}};
use futures::StreamExt;
use reqwest::Response;

pub async fn transcode(req_response: Response) -> std::process::ChildStdout {
    let mut command = Command::new("sh")
        .arg("-c")
        .arg(format!("ffmpeg -i pipe:0 -f wav pipe:1"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped()) // To make it shut the fuck up
        .spawn().expect("Failed to summon ffmpeg");
    
    //command.stdin(Stdio::piped());
    let mut stream = req_response.bytes_stream();

    let mut stdin = command.stdin.take().expect("Failed to open stdin");

    // what the fuck is wrong here
    while let Some(item) = stream.next().await {
        stdin.write(&item.unwrap()).unwrap();
        println!("yeet");
    }
    //stdin.flush().unwrap();
    
    command.stdout.expect("Failed to read from stderr")
}