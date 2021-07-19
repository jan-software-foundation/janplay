extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::fs;
use std::path::Path;

mod func;

#[tokio::main]
async fn main() {
    // Create temp directory
    let temp_dir = format!("{}/jancloud", std::env::temp_dir().to_str().unwrap());
    if !Path::new(&temp_dir).is_dir() {
        fs::create_dir(&temp_dir).expect("Failed to create temp dir");
    }
    
    let mut gen = ulid::Generator::new();
    
    let url = func::fetch_audio_url::fetch_audio_url(String::from("https://www.youtube.com/watch?v=vvDogoFw4B8")).await.unwrap();
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
