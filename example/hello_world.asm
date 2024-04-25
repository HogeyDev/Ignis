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
global _printnum
_printnum:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	mov qword [rbp-8], 0
	mov rax, qword [rbp+16]
	push rax ; recalled `num`
	pop qword rax
	mov qword [rbp-8], rax ; assigned `a`
	mov rax, qword [rbp-8]
	push rax ; recalled `a`
	mov rdx, 0
	push rdx
	pop qword rbx
	pop qword rax
	cmp rax, rbx
	setl al
	movzx rax, al
	push rax
	pop qword rax
	cmp rax, 0
	je lbl2
	mov rdx, 45
	push rdx
	call _putchar
	add rsp, 8
	mov rax, qword [rbp-8]
	push rax ; recalled `a`
	pop qword rax
	neg rax
	push rax
	pop qword rax
	mov qword [rbp-8], rax ; assigned `a`
lbl2:
	mov rax, qword [rbp-8]
	push rax ; recalled `a`
	mov rdx, 9
	push rdx
	pop qword rbx
	pop qword rax
	cmp rax, rbx
	setg al
	movzx rax, al
	push rax
	pop qword rax
	cmp rax, 0
	je lbl3
	mov rax, qword [rbp-8]
	push rax ; recalled `a`
	mov rdx, 10
	push rdx
	pop qword rbx
	pop qword rax
	mov rdx, 0
	div rbx
	push rax
	call _printnum
	add rsp, 8
lbl3:
	mov rdx, 48
	push rdx
	mov rax, qword [rbp-8]
	push rax ; recalled `a`
	mov rdx, 10
	push rdx
	pop qword rbx
	pop qword rax
	mov rdx, 0
	div rbx
	mov rax, rdx
	push rax
	pop qword rbx
	pop qword rax
	add rax, rbx
	push rax
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
global _int_to_usize
_int_to_usize:
	push rbp
	mov rbp, rsp
    mov rax, qword [rbp+16]
	mov rsp, rbp
	pop rbp
	ret
global _main
_main:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	mov qword [rbp-8], 0
	mov rdx, 14
	push rdx
	pop qword rax
	mov qword [rbp-8], rax ; assigned `a`
	sub rsp, 8
	mov qword [rbp-16], 0
	mov rax, qword [rbp-8]
	push rax ; recalled `a`
	call _int_to_usize
	add rsp, 8
	push rax
	pop qword rax
	mov qword [rbp-16], rax ; assigned `b`
	mov rax, qword [rbp-16]
	push rax ; recalled `b`
	call _printnum
	add rsp, 8
	mov rdx, 10
	push rdx
	call _putchar
	add rsp, 8
	mov rdx, 0
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
