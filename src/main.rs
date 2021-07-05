extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod func;

#[tokio::main]
async fn main() {
    
    let url = func::fetch_audio_url::fetch_audio_url(String::from("https://www.youtube.com/watch?v=vvDogoFw4B8")).await.unwrap();
    let resp = reqwest::get(&url)
        .await
        .expect("Failed to download file");
    
    println!("{}", &url);
    
    let stream = func::transcode_audio::transcode(resp).await;
    func::play_stream::play(stream).await;
}
