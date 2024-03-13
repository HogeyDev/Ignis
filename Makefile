CC := gcc
CCARGS := -Wall -Werror -Wpedantic

.PHONY: clean
all: clean compile run

compile:
	$(CC) src/*.c -o build/main -I./src/include $(CCARGS) -g

run:
	./build/main

debug:
	gdb ./build/main

bear:
	bear -- make

clean:
	rm -rf build
	mkdir build
