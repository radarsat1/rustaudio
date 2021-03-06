
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
use std::task::{spawn_sched,SingleThreaded,spawn_with};
use std::comm::{stream,Port,Chan};

mod crtaudio;

enum UGenCommand {
    NoCommand,
    Trigger,
    Quit,
}

type AudioVector = ~([f32]);

type UGenInput = (UGenCommand, AudioVector);

fn sinusoid(port: Port<UGenInput>, chan: Chan<AudioVector>,
            freq: f32, length: int, rate: int) {
    let mut time_idx = length;

    loop {
        /* Wait for the next instruction. */
        let (cmd, imm_samples) = port.recv();
        let mut samples = imm_samples;

        match cmd {
            NoCommand => (),
            Trigger => { time_idx = 0 }
            Quit => break
        }

        for s in samples.mut_iter() {
            let power =
                (std::int::max(length - time_idx, 0) as f32)
                / (length as f32);

            let time = (time_idx as f32) / (rate as f32);

            *s = sin(time * 6.28 * freq) * power * power;

            if (time_idx < length) {
                time_idx += 1;
            }
        }

        /* Send the filled buffer. */
        chan.send(samples);
    }
}

fn main() {
    let (in_port,out_port): (Port<UGenCommand>, Chan<UGenCommand>) = stream();

    /* Audio is processed on a background task. */

    do spawn_sched(SingleThreaded) {
        let rta = crtaudio::CRtAudio::new();

        let (sample_port, sample_chan): (Port<AudioVector>, Chan<AudioVector>) = stream();
        let (cmd_port, cmd_chan): (Port<UGenInput>, Chan<UGenInput>) = stream();

        do spawn_with((cmd_port, sample_chan)) |(cmd_port,sample_chan)|
            { sinusoid(cmd_port, sample_chan, 440.0, 10000, 48000) };

        cmd_chan.send((NoCommand, ~[0.0f32, ..1024]));

        loop {
            let result = sample_port.recv();

            for &s in result.iter() {
                rta.tick( s );
            }

            /* Tell the sinusoid() coroutine what to do based on the
             * command we received, if any. */
            if (in_port.peek()) {
                match (in_port.try_recv()) {
                    Some(x) => match (x) {
                        Quit => { cmd_chan.send((Quit, result)); break },
                        _ => cmd_chan.send((Trigger, result))
                        },
                    None => { cmd_chan.send((Quit, result)); break },
                }
            }
            else {
                cmd_chan.send((NoCommand, result))
            }
        }
    }

    /* Reset the audio task's time counter, re-triggering a pluck
     * sound once a second. */

    for k in range(0,4) {
        do Timer::new().map |mut t| { t.sleep(1000) };
        println(format!("{:d}",k));
        out_port.send(Trigger);
    }

    /* Signal the end of the program to the audio task. */

    do Timer::new().map |mut t| { t.sleep(1000) };
    out_port.send(Quit);
}
