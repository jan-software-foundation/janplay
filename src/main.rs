extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::fs;
use std::io::{stdin,stdout,Write};
use std::path::Path;
use term::color;

mod func;

#[tokio::main]
async fn main() {
    // Create temp directory
    let temp_dir = format!("{}/jancloud", std::env::temp_dir().to_str().unwrap());
    if !Path::new(&temp_dir).is_dir() {
        fs::create_dir(&temp_dir).expect("Failed to create temp dir");
    }
    
    let mut term = term::stdout().unwrap();
    let mut colors: [[u32; 3]; 7] = [
        [216, 160, 223],
        [171, 156, 223],
        [154, 167, 223],
        [143, 223, 211],
        [151, 223, 143],
        [223, 204, 157],
        [223, 168, 168],
    ];
    
    // alright i hate myself for this
    let letters: [[&str; 5]; 7] = [
        [ // J
            "     _",
            "    | |",
            " _  | |",
            "| |_|",
            " \\___"
        ], [ // A
            "   _",
            " / \\",
            "/ _ \\",
            " / ___ \\",
            "/_/   \\_\\"
        ], [ // N
            "    _   _",
            "  | \\ |",
            " |  \\|",
            "| |\\",
            "_| \\_"
        ], [ // P
            " ____",
            " |  _ \\",
            " | |_)",
            "  |  __/",
            "|_|"
        ], [ // L
            "  _",
            "| |",
            " | |",
            "| |___",
            "   |_____"
        ], [ // A
            "        _",
            "      / \\",
            "     / _ \\",
            " / ___ \\",
            "/_/   \\_\\"
        ], [ // Y
            " __   __",
            "\\ \\ / /",
            "\\ V / ",
            "| |  ",
            "_|  "
        ]
    ];
    
    for i in 0..5 {
        for j in 0..7 {
            print!("\x1B[38;2;{};{};{}m", colors[j][0], colors[j][1], colors[j][2]);
            for k in 0..3 {
                colors[j][k] = (colors[j][k] as f32 * 0.9).round() as u32;
            }
            print!("{}", letters[j][i]);
        }
        print!("\n");
    }
    
    term.fg(color::WHITE).unwrap();
    print!("\n");
    print!("Use !quit to quit or !help to list available commands.");
    print!("\n");

    
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
                    let url = func::fetch_audio_url::fetch_audio_url(input).await.unwrap();
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
