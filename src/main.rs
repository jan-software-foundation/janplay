extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::time::Duration;
use std::{fs, thread};
use std::io::{stdin,stdout,Write};
use std::path::Path;

use tokio::task;

mod func;

#[tokio::main]
async fn main() {
    func::print_welcome_message::print();
    
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    
    // Get user input
    loop {
        let mut input = String::new();
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input)
            .expect("Failed to read input");
        
        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }
        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }
        
        match input.trim().to_lowercase().as_str() {
            "!exit" | "!quit" => {
                println!("Exiting.");
                std::process::exit(0);
            }
            "!help" => {
                println!("!quit      Stop playback and exit");
                println!("!pause     Pauses the player");
                println!("!unpause   Resumes a paused player");
                println!("");
                println!("To play audio from YouTube, paste the video URL.");
            }
            "!pause" => {
                if !sink.is_paused() {
                    sink.pause();
                    println!("Player paused.");
                } else {
                    println!("Player is already paused.");
                }
            }
            "!unpause"|"!play" => {
                if sink.is_paused() {
                    sink.play();
                    println!("Player unpaused.");
                } else {
                    println!("Player is not paused.");
                }
            }
            "" => {}
            _ => {
                let url = match func::fetch_audio_url::fetch_audio_url(input).await {
                    Ok(url) => url,
                    _ => {
                        println!("Failed to fetch info from YT");
                        return;
                    }
                };
                let resp = reqwest::get(&url)
                    .await
                    .expect("Failed to download file");
                
                // Create temp directory
                let temp_dir = format!("{}/janplay", std::env::temp_dir().to_str().unwrap());
                if !Path::new(&temp_dir).is_dir() {
                    fs::create_dir(&temp_dir).expect("Failed to create temp dir");
                }
                
                let mut gen = ulid::Generator::new();
                let filename = gen
                    .generate()
                    .expect("Failed to generate ulid")
                    .to_string();
                
                let transcode = task::spawn(func::transcode_audio::transcode(resp, format!("{}/{}", temp_dir, &filename)));
                
                // Wait until ffmpeg creates the file
                while !Path::new(format!("{}/{}", temp_dir, &filename).as_str()).is_file() {
                    thread::sleep(Duration::from_millis(50));
                }
                
                let file = fs::File::open(format!("{}/{}", temp_dir, &filename))
                    .expect("Failed to open file");
                
                func::play_stream::play(file, &sink).await;
                fs::remove_file(format!("{}/{}",temp_dir , &filename))
                    .expect("Failed to delete temporary file");
                
                transcode.await.expect("death");
            }
        }
    }
}
