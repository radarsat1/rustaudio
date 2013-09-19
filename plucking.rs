
/* Program to demonstrate a typical two-task audio control paradigm.
 * A background task calculates audio "plucks" while a control task
 * re-triggers them periodically. */

use std::num::sin;
use std::rt::io::Timer;
use std::io::println;
use std::task::{spawn_with,deschedule};
use std::comm::{stream,Port,Chan};

mod crtaudio;

fn main() {
    let (in_port,out_port): (Port<int>, Chan<int>) = stream();

    /* Audio is processed on a background thread, while must manually
     * yield after each sample. */

    do spawn_with(in_port) |in_port| {
        let rta = crtaudio::CRtAudio::new();
        let mut i = 0;
        while (i != -1) {
            let mut sample = 0.0;

            if (i < 48000) {
                let power = (std::int::max(10000-i, 0) as float)/10000.0 ;
                sample = sin((i as float)/48000.0 * 2.0 * 3.14 * 220.0)
                    * power * power;
                i += 1;
            }

            if (in_port.peek()) {
                match (in_port.try_recv()) {
                    Some(x) => { i = x }
                    None => { i = -1 }
                }
            }

            rta.tick( sample );
            deschedule();
        }
    }

    /* Reset the audio task's `i` variable, re-triggering a pluck
     * sound once a second. */

    let mut k=1;
    while (k <= 3) {
        println(fmt!("%d",k));
        out_port.send(0);
        do Timer::new().map_move |mut t| { t.sleep(1000) };
        k += 1;
    }

    /* A value of -1 signals the end of the program to the audio task. */

    out_port.send(-1);
}
