use rusty_audio::Audio;

pub fn setup_audio() -> Audio {
    let mut audio = Audio::new();
    add_audio("explode", &mut audio);
    add_audio("lose", &mut audio);
    add_audio("move", &mut audio);
    add_audio("pew", &mut audio);
    add_audio("startup", &mut audio);
    add_audio("win", &mut audio);

    audio
}
fn add_audio(name:&str, audio:&mut Audio) {

    let wav = format!("sounds/{}.wav", name);
    audio.add(name, wav);

}

pub fn clear_audio(audio: &Audio){
    audio.wait();
}
