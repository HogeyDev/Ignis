section .text
extern printf
global fibonacci
fibonacci:
	push QWORD [rsp+8]

	push 1
	pop rbx
	pop rax
	push QWORD [rsp+8]

	pop rax
	ret
	push 0
	push 0
	mov QWORD [rsp+0], rax
	push 0
	push 1
	mov QWORD [rsp+8], rax
	push 0
	push 0
	mov QWORD [rsp+16], rax
	push QWORD [rsp+8]

	pop rax
	ret
global _start
_start:
	push 0
	push 10
	mov QWORD [rsp+0], rax
	push QWORD [rsp+0]

	push .STR0
	call printf

section .data
.STR0 db "Fibonacci sequence up to the %dth term:", 0
