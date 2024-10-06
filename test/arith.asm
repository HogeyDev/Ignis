    section .text
global _start
_start:
    push 14 ; 0x0E
    push 18 ; 0x12
    add QWORD [rbp-8], QWORD [rbp-16] ; 32
    pop rax

    mov rdi, rax
    mov rax, 60
    syscall
