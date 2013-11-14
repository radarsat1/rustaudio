
/* Program to demonstrate a typical two-task audio control paradigm.
 * A background task calculates audio "plucks" while a control task
 * re-triggers them periodically. */

/* In this version, the "unit generator" is a function explicitly
 * called with the current sample index. */

use std::num::sin;
use std::rt::io::Timer;
use std::task::{spawn_sched,SingleThreaded};
use std::comm::{stream,Port,Chan};

mod crtaudio;

fn sinusoid(freq: f32, time_idx: int, length: int, rate: int) -> f32
{
    let power =
        (std::int::max(length-time_idx, 0) as f32)
        / (length as f32);

    let sample = sin((time_idx as f32)/(rate as f32) * 6.28 * freq)
        * power * power;

    return sample;
}

fn main() {
    let (in_port,out_port): (Port<int>, Chan<int>) = stream();

    /* Audio is processed on a background thread, while must manually
     * yield after each sample. */

    do spawn_sched(SingleThreaded) {
        let mut i = in_port.recv();
        let rta = crtaudio::CRtAudio::new();
        while (i != -1) {
            let mut sample = 0.0;

            if (i < 10000) {
                sample = sinusoid(440.0, i, 10000, 48000);
                i += 1;
            }

            if (in_port.peek()) {
                match (in_port.try_recv()) {
                    Some(x) => { i = x }
                    None => break
                }
            }

            rta.tick( sample );
        }
    }

    /* Reset the audio task's `i` variable, re-triggering a pluck
     * sound once a second. */

    for k in range(0,4) {
        do Timer::new().map |mut t| { t.sleep(1000) };
        println(format!("{:d}",k));
        out_port.send(0);
    }

    /* A value of -1 signals the end of the program to the audio task. */

    do Timer::new().map |mut t| { t.sleep(1000) };
    out_port.send(-1);
}
