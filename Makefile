# all: run compile_asm run_asm
# 
# run:
# 	cargo run
# 	# RUST_BACKTRACE=1 cargo run
# 	# RUST_BACKTRACE=full cargo run

asmfile = preprocessing

all: compile test

compile:
	cargo build

# compile_asm_win:
# 	nasm -f win64 example/hello_world.asm -o example/hello_world.o
# 	gcc example/hello_world.o -o example/hello_world.exe

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
	gdb --args ./target/debug/ignis -o ./example/$(asmfile).bin ./example/$(asmfile).is

.PHONY: test
test:
	gcc -o example/_test example/test.c -g
	cd example && ./_test
	rm example/_test
