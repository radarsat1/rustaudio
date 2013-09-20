
CC := gcc
CXX := g++
RUSTC := rustc

DEFS := -D__LINUX_ALSA__ -D__LITTLE_ENDIAN__
CFLAGS := -fPIC $(DEFS) -O0 -g
CXXFLAGS := -std=c++11 -fPIC $(DEFS) -O0 -g
LDLIBS := -L. -lcrtaudio -lm

PROGS := sin440 plucking plucking_function plucking_ugen

all: crtaudio/test_crtaudio $(PROGS)

sin440: sin440.rs crtaudio.rs libcrtaudio.so
	$(RUSTC) $<

%: %.rs crtaudio.rs libcrtaudio.so
	$(RUSTC) $<

crtaudio/test_crtaudio: crtaudio/test_crtaudio.o libcrtaudio.so

libcrtaudio.so: crtaudio/crtaudio.o crtaudio/RtAudio.o crtaudio/Stk.o	\
	            crtaudio/RtWvOut.o crtaudio/Mutex.o
	$(CXX) -shared -o $@ $^ -lasound

.PHONY: all clean
clean:
	-@rm -vf *.o crtaudio/*.o libcrtaudio.so crtaudio/test_crtaudio $(PROGS)
