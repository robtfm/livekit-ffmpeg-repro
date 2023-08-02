use ffmpeg_next::format::input;

fn main() {
    ffmpeg_next::init().unwrap();
    input(&"http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4".to_owned()).unwrap();
}

async fn not_used() {
    let _ = livekit::prelude::Room::connect(&"", &"", livekit::prelude::RoomOptions::default()).await.unwrap();
}