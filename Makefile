all: run compile_asm

run:
	cargo run

compile_asm_win:
	nasm -f win64 example/hello_world.asm -o example/hello_world.o
	gcc example/hello_world.o -o example/hello_world.exe

compile_asm:
	nasm -f elf64 example/hello_world.asm -o example/hello_world.o
	gcc example/hello_world.o -o example/hello_world
