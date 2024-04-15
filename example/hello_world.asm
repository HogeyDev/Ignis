section .text
global exit
exit:
	push rbp
	mov rbp, rsp
	mov rax, 60
	mov rdi, qword [rbp+16]
	syscall
	mov rsp, rbp
	pop rbp
	ret
global main
main:
	push rbp
	mov rbp, rsp
	push 1 ; integer literal
	call exit
	add rsp, 8
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
