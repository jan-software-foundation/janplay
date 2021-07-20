extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::fs;
use std::io::{stdin,stdout,Write};
use std::path::Path;

mod func;

#[tokio::main]
async fn main() {
    // Create temp directory
    let temp_dir = format!("{}/janplay", std::env::temp_dir().to_str().unwrap());
    if !Path::new(&temp_dir).is_dir() {
        fs::create_dir(&temp_dir).expect("Failed to create temp dir");
    }
    
    func::print_welcome_message::print();
    
    let mut gen = ulid::Generator::new();
    
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
                println!("there is no help");
            }
            "" => {}
            _ => {
                    let url = match func::fetch_audio_url::fetch_audio_url(input).await {
                        Ok(url) => url,
                        _ => {
                            println!("Failed to fetch info from YT");
                            continue;
                        }
                    };
                    let resp = reqwest::get(&url)
                        .await
                        .expect("Failed to download file");
                    
                    println!("{}", &url);
                    
                    let filename = gen
                        .generate()
                        .expect("Failed to generate ulid")
                        .to_string();
                    
                    func::transcode_audio::transcode(resp, format!("{}/{}",temp_dir , &filename)).await;
                    
                    let file = fs::File::open(format!("{}/{}",temp_dir , &filename))
                        .expect("Failed to open file");
                    
                    func::play_stream::play(file).await;
                    fs::remove_file(format!("{}/{}",temp_dir , &filename))
                        .expect("Failed to delete temporary file");
            }
        }
    }
}
