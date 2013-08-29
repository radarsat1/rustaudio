
CC := gcc
CXX := g++

DEFS := -D__UNIX_JACK__ -D__LITTLE_ENDIAN__
CFLAGS := -fPIC $(DEFS)
CXXFLAGS := -std=c++11 -fPIC $(DEFS)
LDLIBS := -L. -lcrtaudio -ljack -lm

crtaudio/test_crtaudio: crtaudio/test_crtaudio.o libcrtaudio.so

libcrtaudio.so: crtaudio/crtaudio.o crtaudio/RtAudio.o crtaudio/Stk.o	\
	            crtaudio/RtWvOut.o crtaudio/Mutex.o
	$(CXX) -shared -o $@ $^

.PHONY: clean
clean:
	-@rm -vf crtaudio/*.o libcrtaudio.so crtaudio/test_crtaudio
