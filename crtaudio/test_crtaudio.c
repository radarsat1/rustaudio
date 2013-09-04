
#include <stdio.h>
#include <math.h>
#include "crtaudio.h"
#include <unistd.h>

int main()
{
    crtaudio *rta = crtaudio_new();
    if (!rta) {
        printf("Error initializing crtaudio.\n");
        return 1;
    }
    printf("Initialized crtaudio successfully.\n");

    printf("Playing...\n");
    int i;
    for (i=48000; i--; i>0)
        crtaudio_tick(rta, sin(((float)i)/48000.0f * 3.14 * 2 * 440));

    sleep(1);

    printf("Cleanup..\n");
    crtaudio_free(rta);
    return 0;
}
