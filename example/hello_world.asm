section .text
global putchar
putchar:
	push rbp
	mov rbp, rsp
	mov rax, 1
	mov rdi, 1
	movzx rsi, byte [rbp+16]
	mov rdx, 1
	syscall
	mov rsp, rbp
	pop rbp
	ret
global strlen
strlen:
	push rbp
	mov rbp, rsp
	mov rsi, qword [rsp+16]
	mov rax, -1
	.looper:
	inc rax
	cmp byte [rsi+rax], 0x00
	jne .looper
	mov rsp, rbp
	pop rbp
	ret
global print
print:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	mov qword [rbp-8], 0
	push 0 ; integer literal
	pop qword rax
	mov qword [rbp-8], rax ; assigned `i`
lbl0:
	mov rax, qword [rbp+16]
	push rax ; recalled `str`
	mov rax, qword [rbp-8]
	push rax ; recalled `i`
	pop qword rbx
	pop qword rax
	mov rax, [rax + rbx]
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
	mov rax, [rax + rbx]
	push rax
	call putchar
	add rsp, 8
	mov rax, qword [rbp-8]
	push rax ; recalled `i`
	push 1 ; integer literal
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
global println
println:
	push rbp
	mov rbp, rsp
	mov rax, qword [rbp+16]
	push rax ; recalled `str`
	call print
	add rsp, 8
	push 10 ; integer literal
	call putchar
	add rsp, 8
	mov rsp, rbp
	pop rbp
	ret
global main
main:
	push rbp
	mov rbp, rsp
	push STR0
	call println
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
	STR0 db "Hello, World!", 0
