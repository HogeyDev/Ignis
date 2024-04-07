all: run compile_asm

run:
	cargo run

compile_asm:
	nasm -f win64 example/hello_world.asm -o example/hello_world.o
	gcc example/hello_world.o -o hello_world/hello_world.exe
