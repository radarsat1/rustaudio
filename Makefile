
CC := gcc
CXX := g++
RUSTC := rustc

DEFS := -D__UNIX_JACK__ -D__LITTLE_ENDIAN__
CFLAGS := -fPIC $(DEFS)
CXXFLAGS := -std=c++11 -fPIC $(DEFS)
LDLIBS := -L. -lcrtaudio -ljack -lm

all: crtaudio/test_crtaudio sin440

sin440: sin440.rs crtaudio.rs libcrtaudio.so
	$(RUSTC) $<

crtaudio/test_crtaudio: crtaudio/test_crtaudio.o libcrtaudio.so

libcrtaudio.so: crtaudio/crtaudio.o crtaudio/RtAudio.o crtaudio/Stk.o	\
	            crtaudio/RtWvOut.o crtaudio/Mutex.o
	$(CXX) -shared -o $@ $^

.PHONY: all clean
clean:
	-@rm -vf *.o crtaudio/*.o libcrtaudio.so crtaudio/test_crtaudio sin440
