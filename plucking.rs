
/* Program to demonstrate a typical two-task audio control paradigm.
 * A background task calculates audio "plucks" while a control task
 * re-triggers them periodically. */

use std::num::sin;
use std::rt::io::Timer;
use std::io::println;
use std::task::{spawn,deschedule};
use std::comm::{stream,Port,Chan,SharedChan};

mod crtaudio;

fn main() {
    let (out_port,in_port): (Port<int>, Chan<int>) = stream();
    let shared_in_port = SharedChan::new(in_port);

    /* Audio is processed on a background thread, while must manually
     * yield after each sample. */

    do spawn() {
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

            if (out_port.peek()) {
                out_port.try_recv().map_move(|x| { i = x });
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
        shared_in_port.send(0);
        do Timer::new().map_move |mut t| { t.sleep(1000) };
        k += 1;
    }

    /* A value of -1 signals the end of the program to the audio task. */

    shared_in_port.send(-1);
}
