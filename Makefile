
CC := gcc
CXX := g++

DEFS := -D__UNIX_JACK__ -D__LITTLE_ENDIAN__
CFLAGS := -fPIC $(DEFS)
CXXFLAGS := -std=c++11 -fPIC $(DEFS)
LDLIBS := -L. -lcrtaudio -ljack -lm

test_crtaudio: test_crtaudio.o libcrtaudio.so

libcrtaudio.so: crtaudio.o RtAudio.o Stk.o RtWvOut.o Mutex.o
	$(CXX) -shared -o $@ $^

.PHONY: clean
clean:
	-@rm -vf crtaudio.o RtAudio.o libcrtaudio.so test_crtaudio.o test_crtaudio
