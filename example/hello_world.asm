section .text
global _putchar
_putchar:
	push rbp
	mov rbp, rsp
	push QWORD [rbp+16]
	mov rax, 1
	mov rdi, 1
	mov rdx, 1
	mov rsi, rsp
	syscall
	add rsp, 8
	mov rsp, rbp
	pop rbp
	ret
global _strlen
_strlen:
	push rbp
	mov rbp, rsp
	mov rsi, qword [rbp+16]
	mov rax, -1
.looper:
	inc rax
	cmp byte [rsi+rax], 0x00
	jne .looper
	mov rsp, rbp
	pop rbp
	ret
global _print
_print:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	mov qword [rbp-8], 0
	mov rdx, 0
	push rdx
	pop qword rax
	mov qword [rbp-8], rax ; assigned `i`
lbl0:
	mov rax, qword [rbp+16]
	push rax ; recalled `str`
	mov rax, qword [rbp-8]
	push rax ; recalled `i`
	pop qword rbx
	pop qword rax
	imul rbx, 1
	movzx rax, byte [rax + rbx]
	push rax
	pop qword rax
	cmp rax, 0
	je lbl1
	mov rax, qword [rbp+16]
	push rax ; recalled `str`
	mov rax, qword [rbp-8]
	push rax ; recalled `i`
	pop qword rbx
	pop qword rax
	imul rbx, 1
	movzx rax, byte [rax + rbx]
	push rax
	call _putchar
	add rsp, 8
	mov rax, qword [rbp-8]
	push rax ; recalled `i`
	mov rdx, 1
	push rdx
	pop qword rbx
	pop qword rax
	add rax, rbx
	push rax
	pop qword rax
	mov qword [rbp-8], rax ; assigned `i`
	jmp lbl0
lbl1:
	mov rsp, rbp
	pop rbp
	ret
global _println
_println:
	push rbp
	mov rbp, rsp
	mov rax, qword [rbp+16]
	push rax ; recalled `str`
	call _print
	add rsp, 8
	mov rdx, 10
	push rdx
	call _putchar
	add rsp, 8
	mov rsp, rbp
	pop rbp
	ret
global _exit
_exit:
	push rbp
	mov rbp, rsp
	mov rax, 60
	mov rdi, qword [rbp+16]
	syscall
	mov rsp, rbp
	pop rbp
	ret
global _main
_main:
	push rbp
	mov rbp, rsp
	mov rax, STR0
	push rax
	call _println
	add rsp, 8
	mov rdx, 42
	push rdx
	call _exit
	add rsp, 8
	mov rsp, rbp
	pop rbp
	ret

global _start
_start:
	push rbp
	mov rbp, rsp
	call _main
	mov rax, 60
	mov rdi, 0
	syscall
section .data
	STR0 db "Hello, World!", 0
