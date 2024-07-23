section .text
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
global _int_to_char
_int_to_char:
	push rbp
	mov rbp, rsp
    mov rax, qword [rbp+16]
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
global _print
_print:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	mov qword [rbp-8], 0 ; initialized `string_length`
	mov rax, qword [rbp+16]
	push rax ; recalled `str`
	call _strlen
	add rsp, 8
	push rax
	pop rax
	mov qword [rbp-8], rax ; assigned `string_length`
	sub rsp, 8
	mov qword [rbp-16], 0 ; initialized `i`
	mov rdx, 0
	push rdx
	pop rax
	mov qword [rbp-16], rax ; assigned `i`
lbl0:
	mov rax, qword [rbp-16]
	push rax ; recalled `i`
	mov rax, qword [rbp-8]
	push rax ; recalled `string_length`
	pop rbx
	pop rax
	cmp rax, rbx
	setl al
	movzx rax, al
	push rax
	pop rax
	cmp rax, 0
	je lbl1
	sub rsp, 1
	mov byte [rbp-17], 0 ; initialized `c`
	mov rax, qword [rbp+16]
	push rax ; recalled `str`
	mov rax, qword [rbp-16]
	push rax ; recalled `i`
	pop rbx
	pop rax
	imul rbx, 1
	movzx rax, byte [rax + rbx]
	push rax
	pop rax
	mov byte [rbp-17], al ; assigned `c`
	mov al, byte [rbp-17]
	movzx rax, al
	push rax ; recalled `c`
	call _putchar
	add rsp, 8
	mov rax, qword [rbp-16]
	push rax ; recalled `i`
	mov rdx, 1
	push rdx
	pop rbx
	pop rax
	add rax, rbx
	push rax
	mov rax, qword [rsp]
	mov qword [rbp-16], rax
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
	mov qword [rbp-8], 0 ; initialized `a`
	mov rax, qword [rbp+16]
	push rax ; recalled `num`
	pop rax
	mov qword [rbp-8], rax ; assigned `a`
	mov rax, qword [rbp-8]
	push rax ; recalled `a`
	mov rdx, 0
	push rdx
	pop rbx
	pop rax
	cmp rax, rbx
	setl al
	movzx rax, al
	push rax
	pop rax
	cmp rax, 0
	je lbl2
	mov rdx, 45
	push rdx
	call _int_to_char
	add rsp, 8
	movzx rax, al
	push rax
	call _putchar
	add rsp, 8
	mov rax, qword [rbp-8]
	push rax ; recalled `a`
	pop rax
	neg rax
	push rax
	mov rax, qword [rsp]
	mov qword [rbp-8], rax
lbl2:
	mov rax, qword [rbp-8]
	push rax ; recalled `a`
	mov rdx, 9
	push rdx
	pop rbx
	pop rax
	cmp rax, rbx
	setg al
	movzx rax, al
	push rax
	pop rax
	cmp rax, 0
	je lbl3
	mov rax, qword [rbp-8]
	push rax ; recalled `a`
	mov rdx, 10
	push rdx
	pop rbx
	pop rax
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
	pop rbx
	pop rax
	mov rdx, 0
	div rbx
	mov rax, rdx
	push rax
	pop rbx
	pop rax
	add rax, rbx
	push rax
	call _int_to_char
	add rsp, 8
	movzx rax, al
	push rax
	call _putchar
	add rsp, 8
	mov rsp, rbp
	pop rbp
	ret
global _printintpointer
_printintpointer:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	mov qword [rbp-8], 0 ; initialized `as_int`
	mov rdx, 0
	push rdx
	pop rax
	mov qword [rbp-8], rax ; assigned `as_int`
	mov rax, qword [rbp+16]
	mov qword [rbp-8], rax
	mov rax, qword [rbp-8]
	push rax ; recalled `as_int`
	call _printnum
	add rsp, 8
	mov rsp, rbp
	pop rbp
	ret
global _fibonacci_loop
_fibonacci_loop:
	push rbp
	mov rbp, rsp
	mov rax, qword [rbp+16]
	push rax ; recalled `n`
	mov rdx, 2
	push rdx
	pop rbx
	pop rax
	cmp rax, rbx
	setl al
	movzx rax, al
	push rax
	pop rax
	cmp rax, 0
	je lbl4
	mov rax, qword [rbp+16]
	push rax ; recalled `n`
	pop rax
	mov rsp, rbp
	pop rbp
	ret
lbl4:
	sub rsp, 8
	mov qword [rbp-8], 0 ; initialized `prev`
	mov rdx, 0
	push rdx
	pop rax
	mov qword [rbp-8], rax ; assigned `prev`
	sub rsp, 8
	mov qword [rbp-16], 0 ; initialized `curr`
	mov rdx, 1
	push rdx
	pop rax
	mov qword [rbp-16], rax ; assigned `curr`
	sub rsp, 8
	mov qword [rbp-24], 0 ; initialized `temp`
	mov rdx, 0
	push rdx
	pop rax
	mov qword [rbp-24], rax ; assigned `temp`
	sub rsp, 8
	mov qword [rbp-32], 0 ; initialized `i`
	mov rdx, 2
	push rdx
	pop rax
	mov qword [rbp-32], rax ; assigned `i`
lbl5:
	mov rax, qword [rbp-32]
	push rax ; recalled `i`
	mov rax, qword [rbp+16]
	push rax ; recalled `n`
	pop rbx
	pop rax
	cmp rax, rbx
	setle al
	movzx rax, al
	push rax
	pop rax
	cmp rax, 0
	je lbl6
	mov rax, qword [rbp-16]
	push rax ; recalled `curr`
	mov rax, qword [rsp]
	mov qword [rbp-24], rax
	mov rax, qword [rbp-8]
	push rax ; recalled `prev`
	mov rax, qword [rbp-16]
	push rax ; recalled `curr`
	pop rbx
	pop rax
	add rax, rbx
	push rax
	mov rax, qword [rsp]
	mov qword [rbp-16], rax
	mov rax, qword [rbp-24]
	push rax ; recalled `temp`
	mov rax, qword [rsp]
	mov qword [rbp-8], rax
	mov rax, qword [rbp-32]
	push rax ; recalled `i`
	mov rdx, 1
	push rdx
	pop rbx
	pop rax
	add rax, rbx
	push rax
	mov rax, qword [rsp]
	mov qword [rbp-32], rax
	jmp lbl5
lbl6:
	mov rax, qword [rbp-16]
	push rax ; recalled `curr`
	pop rax
	mov rsp, rbp
	pop rbp
	ret
	mov rsp, rbp
	pop rbp
	ret
global _fibonacci_recursive
_fibonacci_recursive:
	push rbp
	mov rbp, rsp
	mov rax, qword [rbp+16]
	push rax ; recalled `n`
	mov rdx, 2
	push rdx
	pop rbx
	pop rax
	cmp rax, rbx
	setl al
	movzx rax, al
	push rax
	pop rax
	cmp rax, 0
	je lbl7
	mov rax, qword [rbp+16]
	push rax ; recalled `n`
	pop rax
	mov rsp, rbp
	pop rbp
	ret
lbl7:
	mov rax, qword [rbp+16]
	push rax ; recalled `n`
	mov rdx, 1
	push rdx
	pop rbx
	pop rax
	sub rax, rbx
	push rax
	call _fibonacci_recursive
	add rsp, 8
	push rax
	mov rax, qword [rbp+16]
	push rax ; recalled `n`
	mov rdx, 2
	push rdx
	pop rbx
	pop rax
	sub rax, rbx
	push rax
	call _fibonacci_recursive
	add rsp, 8
	push rax
	pop rbx
	pop rax
	add rax, rbx
	push rax
	pop rax
	mov rsp, rbp
	pop rbp
	ret
	mov rsp, rbp
	pop rbp
	ret
global _main
_main:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	mov qword [rbp-8], 0 ; initialized `index`
	mov rdx, 7
	push rdx
	pop rax
	mov qword [rbp-8], rax ; assigned `index`
	sub rsp, 8
	mov qword [rbp-16], 0 ; initialized `rec`
	mov rax, qword [rbp-8]
	push rax ; recalled `index`
	call _fibonacci_recursive
	add rsp, 8
	push rax
	pop rax
	mov qword [rbp-16], rax ; assigned `rec`
	sub rsp, 8
	mov qword [rbp-24], 0 ; initialized `looped`
	mov rax, qword [rbp-8]
	push rax ; recalled `index`
	call _fibonacci_loop
	add rsp, 8
	push rax
	pop rax
	mov qword [rbp-24], rax ; assigned `looped`
	mov rax, qword [rbp-16]
	push rax ; recalled `rec`
	mov rax, qword [rbp-24]
	push rax ; recalled `looped`
	pop rbx
	pop rax
	sub rax, rbx
	push rax
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