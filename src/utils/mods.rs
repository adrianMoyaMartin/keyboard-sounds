use std::path::Path;
use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};

pub fn create_variables()->(Vec<Vec<f32>>, OutputStreamHandle) {
    // loads all custom sounds
    let mut audio_data: Vec<Vec<f32>> = Vec::with_capacity(256);
    for i in 0..255 {
        let file_path = format!("src\\audio\\{}.wav", i);
        let path = Path::new(&file_path);
        if path.exists() {
            audio_data.push(save_audio(&file_path));
        } else {
            audio_data.push(save_audio("src\\audio\\press.wav"));
        }
    }
    // creates stream handle
    let (_, stream_handle) = OutputStream::try_default().unwrap();

    return (audio_data, stream_handle)
}

pub fn play_sound(audio_data: &Vec<Vec<f32>>, stream_handle: &OutputStreamHandle, i: usize) {
    let source_instance = rodio::buffer::SamplesBuffer::new(
        1,
        55150,
        &*audio_data[i]
    );
    stream_handle
        .play_raw(source_instance.convert_samples())
        .expect("failed to play sounds");
}

fn save_audio(path: &str) -> Vec<f32> {
    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();
    source.convert_samples::<f32>().collect()
}
