#ifndef _CRTAUDIO_H_
#define _CRTAUDIO_H_

typedef void crtaudio;

crtaudio *crtaudio_new();
void crtaudio_free(crtaudio *rta);
void crtaudio_tick(crtaudio *rta, float sample);

#endif
