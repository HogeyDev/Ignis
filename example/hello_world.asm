section .text
global _main
_main:
	push rbp
	mov rbp, rsp
	sub rsp, 16
	mov qword [rbp-8], 0
	mov qword [rbp-16], 0
	mov rax, qword [rbp-16] ; 16 + 0
	mov rdx, 14
	push rdx
	pop rbx
	pop rax
	mov qword [rbp-16], rbx
	push rax
	mov rax, qword [rbp-16] ; 16 + 0
	pop rax
	mov rsp, rbp
	pop rbp
	ret
	mov rsp, rbp
	pop rbp
	ret

global _start
_start:
	push rbp
	mov rbp, rsp
	call _main
	mov rdi, rax
	mov rax, 60
	syscall
section .data
