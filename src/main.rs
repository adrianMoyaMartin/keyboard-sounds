use winapi::um::winuser;

mod utils {
    pub mod mods;
}
use crate::utils::mods;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut last_state: [bool; 255] = [false; 255];

    let (audio_data, _stream, stream_handle) = mods::create_variables();

    loop {
        for i in 0..255 {
            let state = unsafe { winuser::GetAsyncKeyState(i) };

            if (state & (0x8000u16 as i16)) != 0 {
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

        sleep(Duration::from_millis(5));
    }
}
