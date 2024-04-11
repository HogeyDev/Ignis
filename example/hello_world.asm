section .text
global print
print:
	push rbp
	mov rbp, rsp
mov rdi, QWORD [rsp+16]
	xor rax, rax
	mov rsp, rbp
	pop rbp
	ret
global println
println:
	push rbp
	mov rbp, rsp
	push QWORD [rsp+16]
	call print
	mov rsp, rbp
	pop rbp
	ret
global main
main:
	push rbp
	mov rbp, rsp
	push STR0
	call println
	mov rsp, rbp
	pop rbp
	ret

global _start
_start:
	push rbp
	mov rbp, rsp
	call main
	mov rax, 60
	mov rdi, 0
	syscall
section .data
	STR0 db "Hello, World!", 0
