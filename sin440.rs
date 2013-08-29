
use std::num::sin;

mod crtaudio;

fn main() {
    let rta = crtaudio::crtaudio_new();
    let mut i = 0;
    while (i < 48000) {
        crtaudio::crtaudio_tick(rta,
            sin((i as float)/48000.0 * 2.0 * 3.14 * 440.0));
        i += 1;
    }
    crtaudio::crtaudio_free(rta);
}
