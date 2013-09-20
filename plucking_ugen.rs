
/* Program to demonstrate a typical two-task audio control paradigm.
 * A background task calculates audio "plucks" while a control task
 * re-triggers them periodically. */

/* In this version, the "unit generator" is a UGen trait with a tick()
 * function that tracks time itself.  Additionally, a UGenTrigger
 * trait enables a unit generator that can be "triggered", i.e., no
 * pitch or volume information is needed, simply the sinusoid counter
 * is reset. */

use std::num::sin;
use std::rt::io::Timer;
use std::io::println;
use std::task::{spawn_sched,SingleThreaded};
use std::comm::{stream,Port,Chan};

mod crtaudio;

trait UGen {
    fn tick(&mut self) -> float;
}

trait UGenTrigger {
    fn trigger(&mut self);
}

struct PluckedSinusoid {
    freq: float,
    time_idx: int,
    length: int,
    rate: int,
}

impl PluckedSinusoid {
    fn new(freq: float, length: int, rate: int) -> PluckedSinusoid {
        return PluckedSinusoid {
            freq: freq, time_idx: length, length: length, rate: rate
        }
    }
}

impl UGen for PluckedSinusoid {
    fn tick(&mut self) -> float {
        let power =
            (std::int::max(self.length - self.time_idx, 0) as float)
            / (self.length as float);

        let time = (self.time_idx as float) / (self.rate as float);

        let sample = sin(time * 6.28 * self.freq)
            * power * power;

        if (self.time_idx < self.length) {
            self.time_idx += 1;
        }

        return sample;
    }
}

impl UGenTrigger for PluckedSinusoid {
    fn trigger(&mut self) {
        self.time_idx = 0;
    }
}

fn main() {
    let (in_port,out_port): (Port<int>, Chan<int>) = stream();

    /* Audio is processed on a background thread, while must manually
     * yield after each sample. */

    do spawn_sched(SingleThreaded) {
        let rta = crtaudio::CRtAudio::new();

        let mut sinusoid = PluckedSinusoid::new(440.0, 10000, 48000);

        loop {
            let sample = sinusoid.tick();
            rta.tick( sample );

            if (in_port.peek()) {
                match (in_port.try_recv()) {
                    Some(x) => match (x) {
                        -1 => break,
                        _ => sinusoid.trigger()
                        },
                    None => break
                }
            }
        }
    }

    /* Reset the audio task's `i` variable, re-triggering a pluck
     * sound once a second. */

    for k in range(0,4) {
        do Timer::new().map_move |mut t| { t.sleep(1000) };
        println(fmt!("%d",k));
        out_port.send(0);
    }

    /* A value of -1 signals the end of the program to the audio task. */

    do Timer::new().map_move |mut t| { t.sleep(1000) };
    out_port.send(-1);
}
