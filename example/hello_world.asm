section .text
global _main
_main:
	push rbp
	mov rbp, rsp
	sub rsp, 16
	mov qword [rbp-16], 0
	mov qword [rbp-24], 0
	push 0 ; zero-initialize dynamic array
	push 0 ; zero-initialize primative
	mov rdx, 0
	push rdx
	push 0 ; zero-initialize dynamic array
	push 0 ; zero-initialize primative
	mov rdx, 0
	push rdx
	mov rax, qword [rbp-48]
	mov qword [rbp-16], rax
	mov rax, qword [rbp-40]
	mov qword [rbp-8], rax
	add rsp, 16
	mov rax, qword [rbp-24]
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
