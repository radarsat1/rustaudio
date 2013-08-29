
#[link(name = "crtaudio", vers = "2.5")];

use std::libc::{c_float};

mod _crtaudio {
    use std::libc::{c_float,c_void};
    pub type crtaudio = *c_void;

    #[link_args = "-lcrtaudio -ljack"]
    extern {
        fn crtaudio_new() -> crtaudio;
        fn crtaudio_free(rta: crtaudio);
        fn crtaudio_tick(rta: crtaudio, sample: c_float);
    }
}

type crtaudio = _crtaudio::crtaudio;

#[fixed_stack_segment]
#[inline(never)]
pub fn crtaudio_new() -> crtaudio
{
    unsafe { return _crtaudio::crtaudio_new(); }
}

#[fixed_stack_segment]
#[inline(never)]
pub fn crtaudio_free(rta: crtaudio)
{
    unsafe { _crtaudio::crtaudio_free(rta); }
}

#[fixed_stack_segment]
#[inline(never)]
pub fn crtaudio_tick(rta: crtaudio, sample: float)
{
    unsafe { _crtaudio::crtaudio_tick(rta, sample as c_float); }
}
