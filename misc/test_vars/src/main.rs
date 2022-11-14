mod media;
use media::Playable;

struct Audio(String);
struct Video(String);

impl Playable for Audio {
    fn play(&self){
        println!("playing: {}",self.0);
    }
}
impl Playable for Video{
    fn play(&self){
        println!("playing: {}",self.0);
    }
}

fn main() {
    let audio = Audio("audiomp3".to_string());
    let video = Video("vodemv4".to_string());
    audio.play();
    video.play();
}
