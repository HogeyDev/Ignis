# all: run compile_asm run_asm
# 
# run:
# 	cargo run
# 	# RUST_BACKTRACE=1 cargo run
# 	# RUST_BACKTRACE=full cargo run

all: compile test

compile:
	cargo build

# compile_asm_win:
# 	nasm -f win64 example/hello_world.asm -o example/hello_world.o
# 	gcc example/hello_world.o -o example/hello_world.exe

compile_asm:
	nasm -f elf64 example/hello_world.asm -o example/hello_world.o -g
	@# ld -m elf_x86_64 -dynamic-linker /lib64/ld-linux-x86-64.so.2 example/hello_world.o -o example/hello_world -lc # link with libc
	ld -m elf_x86_64 example/hello_world.o -o example/hello_world
	rm example/hello_world.o

run_asm:
	./example/hello_world

debug_asm:
	gdb ./example/hello_world

debug:
	gdb ./target/debug/ignis

test:
	gcc -o example/test example/test.c -g
	cd example && ./test && rm test
