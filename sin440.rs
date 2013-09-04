
use std::num::sin;

mod crtaudio;

fn main() {
    let rta = crtaudio::CRtAudio::new();
    let mut i = 0;
    while (i < 48000) {
        rta.tick( sin((i as float)/48000.0 * 2.0 * 3.14 * 440.0) );
        i += 1;
    }
}
