#ifndef _CRTAUDIO_H_
#define _CRTAUDIO_H_

typedef void crtaudio;

#ifdef __cplusplus
extern "C" {
#endif

crtaudio *crtaudio_new();
void crtaudio_free(crtaudio *rta);
void crtaudio_tick(crtaudio *rta, float sample);

#ifdef __cplusplus
}
#endif

#endif
