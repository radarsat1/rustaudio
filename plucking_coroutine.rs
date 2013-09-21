
/* Program to demonstrate a typical two-task audio control paradigm.
 * A background task calculates audio "plucks" while a control task
 * re-triggers them periodically. */

/* In this version, the "unit generator" is encapsulated as a
 * coroutine implemented using a task and communication channel.  The
 * communication channel is used to pass audio vectors of several
 * samples, which seems to be efficient enough for this to run in real
 * time. */

use std::num::sin;
use std::rt::io::Timer;
use std::io::println;
use std::task::{spawn_sched,SingleThreaded};
use std::comm::{stream,Port,Chan};

use coroutine::coroutine;

mod crtaudio;
mod coroutine;

enum UGenCommand {
    NoCommand,
    Trigger,
    Quit,
}

type AudioVector = ~([float]);

type UGenInput = (UGenCommand, AudioVector);

/* Return a coroutine that yields decaying sinusoid samples.  Can be
 * sent a command to re-trigger the decay. */
fn sinusoid(freq: float, length: int, rate: int, buffer: AudioVector)
            -> ~fn:Send((UGenCommand,~[float])) -> ~[float] {
    coroutine((freq, length, rate, buffer),
        |(freq, length, rate, buffer), coyield| {
            let mut time_idx = length;
            let mut samples = buffer;

            loop {
                for s in samples.mut_iter() {
                    let power =
                        (std::int::max(length - time_idx, 0) as float)
                        / (length as float);

                    let time = (time_idx as float) / (rate as float);

                    *s = sin(time * 6.28 * freq) * power * power;

                    if (time_idx < length) {
                        time_idx += 1;
                    }
                }

                /* Send buffer & wait for the next instruction. */
                let (cmd, imm_samples): (UGenCommand,AudioVector) = coyield(samples);
                samples = imm_samples;

                match cmd {
                    NoCommand => (),
                    Trigger => { time_idx = 0 }
                    Quit => break
                }
            }
        })
}

fn main() {
    let (in_port,out_port): (Port<UGenCommand>, Chan<UGenCommand>) = stream();

    /* Audio is processed on a background task. */

    do spawn_sched(SingleThreaded) {
        let rta = crtaudio::CRtAudio::new();

        let buffer1 = ~[0.0f, ..1024];
        let mut buffer = ~[0.0f, ..1024];
        let ugen = sinusoid(440.0, 10000, 48000, buffer1);

        loop {
            /* Tell the sinusoid() coroutine what to do based on the
             * command we received, if any. */
            let cmd = if (in_port.peek()) {
                match (in_port.recv()) {
                    Quit => Quit,
                    _ => Trigger
                }
            }
            else {
                NoCommand
            };

            let result = ugen((cmd, buffer));

            for &s in result.iter() {
                rta.tick( s );
            }

            /* Pass the previous buffer back in the next time around. */
            buffer = result;

            match cmd { Quit => break, _ => () };
        }
    }

    /* Reset the audio task's time counter, re-triggering a pluck
     * sound once a second. */

    for k in range(0,4) {
        do Timer::new().map_move |mut t| { t.sleep(1000) };
        println(fmt!("%d",k));
        out_port.send(Trigger);
    }

    /* Signal the end of the program to the audio task. */

    do Timer::new().map_move |mut t| { t.sleep(1000) };
    out_port.send(Quit);
}
