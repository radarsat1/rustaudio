
#include <stdlib.h>
#include <stdio.h>

#include "RtWvOut.h"

#include <memory>

typedef stk::RtWvOut crtaudio;

extern "C" {

crtaudio *crtaudio_new()
{
    stk::Stk::setSampleRate(48000);
    return new stk::RtWvOut();
}

void crtaudio_free(crtaudio *rta)
{
    delete rta;
}

void crtaudio_tick(crtaudio *rta, float sample)
{
    rta->tick(sample);
}

}
