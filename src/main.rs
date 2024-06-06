use winapi::um::winuser;

use std::{ path::Path, thread::sleep };
use std::time::Duration;

use std::fs::File;
use std::io::BufReader;
use rodio::{ source::Source, Decoder, OutputStream };

fn main() {
    let mut last_state: [bool; 255] = [false; 255];

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let time = std::time::Instant::now();

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
    println!("{} ms taken to load keys", time.elapsed().as_millis());

    loop {
        for i in 0..255 {
            let state = unsafe { winuser::GetAsyncKeyState(i) };

            if (state & (0x8000u16 as i16)) != 0 {
                if last_state[i as usize] == false && i != 1 && i != 2 && i != 3 {
                    let source_instance = rodio::buffer::SamplesBuffer::new(
                        1,
                        55150,
                        &*audio_data[i as usize]
                    );
                    stream_handle
                        .play_raw(source_instance.convert_samples())
                        .expect("failed to play sounds");

                    println!("Key with virtual-key code {} is pressed.", i);
                }
                last_state[i as usize] = true;
            } else {
                last_state[i as usize] = false;
            }
        }

        sleep(Duration::from_millis(5));
    }
}
fn save_audio(path: &str) -> Vec<f32> {
    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();
    source.convert_samples::<f32>().collect()
}
