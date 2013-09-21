
use std::task::spawn_with;
use std::comm::{stream,Port,Chan};

pub fn coroutine<Tin:Send, Tout:Send, A:Send>(args: A, func: ~fn(A, ~fn(Tout)->Tin)) -> (~fn(Tin)->Tout) {
    let (in_port, in_chan): (Port<Tin>, Chan<Tin>) = stream();
    let (out_port, out_chan): (Port<Tout>, Chan<Tout>) = stream();

    let in_port_ref: ~Port<Tin> = ~in_port;
    let in_chan_ref: ~Chan<Tin> = ~in_chan;
    let out_port_ref: ~Port<Tout> = ~out_port;
    let out_chan_ref: ~Chan<Tout> = ~out_chan;

    let coyield:~fn:Send(Tout) -> Tin = |value: Tout| -> Tin {
        out_chan_ref.send(value);
        in_port_ref.recv()
    };

    spawn_with((args, coyield), |(a,c)| func(a,c));

    // Return a function that can be used to communicate with this coroutine
    |x| { in_chan_ref.send(x); out_port_ref.recv() }
}

fn test_coroutines()
{
    let sendto = coroutine(10,
        |x: int, coyield: ~fn(float) -> int| {
            let y = x as float;
            println(fmt!("coroutine sending %f",y));
            let z = coyield(y);
            println(fmt!("coroutine got %d", z));
        });

    let x = sendto(20);
    println(fmt!("main task sent 20, got %f", x));
}
