
#[link(name = "crtaudio", vers = "2.5")];

use std::libc::{c_float};

mod _crtaudio {
    use std::libc::{c_float,c_void};
    pub type crtaudio = *c_void;

    #[link_args = "-lcrtaudio -lasound"]
    extern {
        pub fn crtaudio_new() -> crtaudio;
        pub fn crtaudio_free(rta: crtaudio);
        pub fn crtaudio_tick(rta: crtaudio, sample: c_float);
    }
}

struct CRtAudio {
    handle: _crtaudio::crtaudio
}

impl CRtAudio {
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> CRtAudio {
        CRtAudio { handle: unsafe { _crtaudio::crtaudio_new() } }
    }

    #[fixed_stack_segment]
    #[inline(never)]
    pub fn tick(&self, sample: f32) {
        unsafe { _crtaudio::crtaudio_tick(self.handle,
                                          sample as c_float) };
    }
}

impl Drop for CRtAudio {
    #[fixed_stack_segment]
    #[inline(never)]
    fn drop(&mut self) {
        unsafe { _crtaudio::crtaudio_free(self.handle); }
    }
}
