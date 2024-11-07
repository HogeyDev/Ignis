section .text
global _start
_start:
    mov rax, STR0
    mov al, byte [rax+1]
    movzx rax, al

    mov rax, 60
    mov rdi, 0
    syscall
    
section .data
STR0: db "Hello, World!", 0
