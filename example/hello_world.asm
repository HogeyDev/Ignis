section .text
global putchar
putchar:
	push rbp
	mov rbp, rsp
mov rax, 1
mov rdi, 1
mov rsi, [rsp+16]
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
	push 0 ; created `i`
	push 0 ; integer literal
	pop rax
	mov qword [rsp+0], rax ; assigned `i`
lbl0:
	mov rax, qword [rsp+24]
	push rax ; recalled `str`
	mov rax, qword [rsp+8]
	push rax ; recalled `i`
	pop rbx
	pop rax
	mov rax, [rax + rbx]
	push rax
	pop rax
	cmp rax, 0
	je lbl1
	mov rax, qword [rsp+24]
	push rax ; recalled `str`
	mov rax, qword [rsp+8]
	push rax ; recalled `i`
	pop rbx
	pop rax
	mov rax, [rax + rbx]
	push rax
	call putchar
	add rsp, 8
	push 10 ; integer literal
	call putchar
	add rsp, 8
	mov rax, qword [rsp+16]
	push rax ; recalled `i`
	push 1 ; integer literal
	pop rbx
	pop rax
	add rax, rbx
	push rax
	pop rax
	mov qword [rsp+16], rax ; assigned `i`
	jmp lbl0
lbl1:
	mov rsp, rbp
	pop rbp
	ret
global println
println:
	push rbp
	mov rbp, rsp
	mov rax, qword [rsp+16]
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
