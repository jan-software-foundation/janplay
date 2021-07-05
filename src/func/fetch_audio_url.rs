use std::thread;
use std::sync::mpsc;
use youtube_dl::YoutubeDl;

pub async fn fetch_audio_url(url: String) -> Result<String, ()> {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let ytdl = YoutubeDl::new(&url)
            .socket_timeout("15")
            .run()
            .unwrap();
        
        let ytdl_str = serde_json::to_string(&ytdl).unwrap();
        let json: serde_json::Value = serde_json::from_str(&ytdl_str).unwrap();
        let video_formats = json.get("SingleVideo")
            .unwrap()
            .get("formats")
            .unwrap()
            .as_array()
            .unwrap();
        
        let mut h: i32 = 0;
        let mut stream_url = "";
        
        for e in video_formats {
            if e.get("vcodec").unwrap() == "none" && e.get("acodec").unwrap() != "none" {
                let format_id: i32 = e.get("format_id").unwrap().as_str().unwrap().parse::<i32>().unwrap();
                
                if format_id > h {
                    h = format_id;
                    stream_url = e.get("url").unwrap().as_str().unwrap();
                }
            }
        }
        
        if stream_url != "" {
            tx.send(String::from(stream_url)).unwrap();
        }
    });
    
    Ok(rx.recv().unwrap())
}