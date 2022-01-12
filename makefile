CC = gcc
CCFLAGS = -Wall -Wextra -Iinc
BINARY = bin/main

SOURCES = $(wildcard src/*.c)
OBJECTS = $(patsubst src/%.c,bin/%.o,$(SOURCES))
DEPENDS = $(patsubst src/%.c,bin/%.d,$(SOURCES))

.PHONY: default
default: debug

debug: CCFLAGS += -g3 -O0
debug: BINARY = bin/main.debug
debug: build

release: CCFLAGS += -O3 -g0
release: BINARY = bin/main.release
release: build

clean:
	rm -f bin/*

bin/%.o: src/%.c
	$(CC) $(CCFLAGS) -MMD -MP -MF bin/$*.d -c -o $@ $<

-include $(DEPENDS)

build: $(OBJECTS)
	$(CC) $^ -o $(BINARY)
