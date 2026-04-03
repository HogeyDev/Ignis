.PHONY: test compile long install debug debug_asm run_asm compile_asm
asmfile = new_print

all: compile test

compile:
	zig build-exe -femit-bin=./build/ignis src/main.zig
long:
	zig build-exe -femit-bin=./build/ignis src/main.zig -freference-trace=$(depth)

compile_asm:
	nasm -f elf64 example/$(asmfile).bin.asm -o example/$(asmfile).o -g
	@# ld -m elf_x86_64 -dynamic-linker /lib64/ld-linux-x86-64.so.2 example/fibonacci.o -o example/fibonacci -lc # link with libc
	ld -m elf_x86_64 example/$(asmfile).o -o example/$(asmfile).bin
	rm example/$(asmfile).o

run_asm:
	./example/$(asmfile).bin

debug_asm:
	gdb ./example/$(asmfile).bin

debug:
	gdb --args ./build/ignis -o ./example/$(asmfile).bin ./example/$(asmfile).is

test:
	gcc -o example/_test example/test.c -g
	cd example && ./_test
	rm example/_test

install:
	cp ./target/release/ignis /usr/local/bin/
