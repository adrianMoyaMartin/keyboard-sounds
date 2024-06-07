use winapi::um::winuser;

mod utils {
    pub mod mods;
}
use crate::utils::mods;

use std::thread::sleep;
use std::time::Instant;
use std::time::Duration;

fn main() {
    let mut last_state: [bool; 255] = [false; 255];

    let (audio_data, _stream, stream_handle) = mods::create_variables();
    let mut last_touched = Instant::now();

    loop {
        for i in 0..255 {
            let state = unsafe { winuser::GetAsyncKeyState(i) };

            if (state & (0x8000u16 as i16)) != 0 {
                last_touched = Instant::now();
                if last_state[i as usize] == false && i != 1 && i != 2 && i != 3 {
                    mods::play_sound(&audio_data, &_stream, &stream_handle, i as usize);
                }
                // saves value as being hold
                last_state[i as usize] = true;
            } else {
                // saves value as not being hold
                last_state[i as usize] = false;
            }
        }
        if last_touched.elapsed().as_millis() >= 5000 {
            sleep(Duration::from_millis(50));
        } else {
            sleep(Duration::from_millis(5));
        }
    }
}
