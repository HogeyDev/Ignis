all: run compile_asm run_asm

run:
	cargo run

# compile_asm_win:
# 	nasm -f win64 example/hello_world.asm -o example/hello_world.o
# 	gcc example/hello_world.o -o example/hello_world.exe

compile_asm:
	nasm -f elf64 example/hello_world.asm -o example/hello_world.o -g
	ld -m elf_x86_64 -dynamic-linker /lib64/ld-linux-x86-64.so.2 example/hello_world.o -o example/hello_world -lc
	rm example/hello_world.o

run_asm:
	./example/hello_world
