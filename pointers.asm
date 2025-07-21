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
global _read
_read:
	push rbp
	mov rbp, rsp
	mov rdx, STR0
	push rdx
	call _println
	add rsp, 8
	mov rdx, 1
	push rdx
	call _exit
	add rsp, 8
	mov rsp, rbp
	pop rbp
	ret
	mov rdx, 0
	push rdx
	mov rax, qword [rsp+0]
	mov qword [GLO1+0], rax
	add rsp, 8
global _mmap
_mmap:
	push rbp
	mov rbp, rsp
	mov rax, 9
	mov rdi, qword [rbp+56]
	mov rsi, qword [rbp+48]
	mov rdx, qword [rbp+40]
	mov r10, qword [rbp+32]
	mov r8, qword [rbp+24]
	mov r9, qword [rbp+16]
	syscall
	mov qword [rbp-8], rax
	mov rsp, rbp
	pop rbp
	ret
global _malloc
_malloc:
	push rbp
	mov rbp, rsp
	sub rsp, 8 ; stack reserved for `top`
	mov qword [rsp+0], 0
	mov rax, qword [GLO1+0]
	push rax ; recalled `heap_pointer`
	mov rax, qword [rsp+0]
	mov qword [rbp-8], rax
	add rsp, 8
	mov rax, qword [GLO1+0]
	push rax ; recalled `heap_pointer`
	mov rax, qword [rbp+16]
	push rax ; recalled `bytes`
	pop rbx
	pop rax
	add rax, rbx
	push rax
	lea rdx, qword [GLO1+0]
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
	mov rax, qword [rbp-8]
	push rax ; recalled `top`
	pop rcx
	lea rdx, qword [GLO0+0+rcx*1] ; rcx is multiplied to adjust for type sizing
	mov rax, rdx
	push rax
	pop rax
	mov rsp, rbp
	pop rbp
	ret
	mov rsp, rbp
	pop rbp
	ret
global _free
_free:
	push rbp
	mov rbp, rsp
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
global _format
_format:
	push rbp
	mov rbp, rsp
	mov rsp, rbp
	pop rbp
	ret
global _print_char
_print_char:
	push rbp
	mov rbp, rsp
   movzx rax, byte [rbp+16]
    push rax
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
	sub rsp, 8 ; stack reserved for `string_length`
	mov qword [rsp+0], 0
	mov rax, qword [rbp+16]
	push rax ; recalled `str`
	call _strlen
	add rsp, 8
	push rax
	mov rax, qword [rsp+0]
	mov qword [rbp-8], rax
	add rsp, 8
	sub rsp, 8 ; stack reserved for `i`
	mov qword [rsp+0], 0
	mov rdx, 0
	push rdx
	mov rax, qword [rsp+0]
	mov qword [rbp-16], rax
	add rsp, 8
lbl2:
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
	je lbl3
	sub rsp, 1 ; stack reserved for `c`
	mov byte [rsp+0], 0
	mov rax, qword [rbp-16]
	push rax ; recalled `i`
	pop rbx
	mov rax, qword [rbp+16]
	imul rbx, 1
	add rax, rbx
	sub rsp, 1 ; allocated space to push into
	mov al, byte [rax+0]
	mov byte [rsp+0], al
	mov al, byte [rsp+0]
	mov byte [rbp-17], al
	add rsp, 1
	mov al, byte [rbp-17]
	movzx rax, al
	push rax ; recalled `c`
	call _print_char
	add rsp, 8
	mov rax, qword [rbp-16]
	push rax ; recalled `i`
	mov rdx, 1
	push rdx
	pop rbx
	pop rax
	add rax, rbx
	push rax
	lea rdx, qword [rbp-16]
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
	jmp lbl2
lbl3:
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
	call _print_char
	add rsp, 8
	mov rsp, rbp
	pop rbp
	ret
global _print_int
_print_int:
	push rbp
	mov rbp, rsp
	sub rsp, 8 ; stack reserved for `a`
	mov qword [rsp+0], 0
	mov rax, qword [rbp+16]
	push rax ; recalled `num`
	mov rax, qword [rsp+0]
	mov qword [rbp-8], rax
	add rsp, 8
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
	je lbl4
	mov rdx, 45
	push rdx
	call _int_to_char
	add rsp, 8
	movzx rax, al
	push rax
	call _print_char
	add rsp, 8
	mov rax, qword [rbp-8]
	push rax ; recalled `a`
	pop rax
	neg rax
	push rax
	lea rdx, qword [rbp-8]
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
lbl4:
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
	je lbl5
	mov rax, qword [rbp-8]
	push rax ; recalled `a`
	mov rdx, 10
	push rdx
	pop rbx
	pop rax
	mov rdx, 0
	div rbx
	push rax
	call _print_int
	add rsp, 8
lbl5:
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
	call _print_char
	add rsp, 8
	mov rsp, rbp
	pop rbp
	ret
global _print_int_pointer
_print_int_pointer:
	push rbp
	mov rbp, rsp
	sub rsp, 8 ; stack reserved for `as_int`
	mov qword [rsp+0], 0
	mov rdx, 0
	push rdx
	mov rax, qword [rsp+0]
	mov qword [rbp-8], rax
	add rsp, 8
	mov rax, qword [rbp+16]
	mov qword [rbp-8], rax
	mov rax, qword [rbp-8]
	push rax ; recalled `as_int`
	call _print_int
	add rsp, 8
	mov rsp, rbp
	pop rbp
	ret
global _main
_main:
	push rbp
	mov rbp, rsp
	sub rsp, 8 ; stack reserved for `x`
	mov qword [rsp+0], 0
	mov rdx, 42
	push rdx
	mov rax, qword [rsp+0]
	mov qword [rbp-8], rax
	add rsp, 8
	sub rsp, 8 ; stack reserved for `y`
	mov qword [rsp+0], 0
	lea rdx, qword [rbp-8]
	mov rax, rdx
	push rax
	mov rax, qword [rsp+0]
	mov qword [rbp-16], rax
	add rsp, 8
	sub rsp, 8 ; stack reserved for `z`
	mov qword [rsp+0], 0
	mov rax, qword [rbp-16]
	push rax ; recalled `y`
	pop rax
	sub rsp, 8
	mov rax, qword [rax+0]
	mov qword [rsp+0], rax
	push rax
	mov rax, qword [rsp+0]
	mov qword [rbp-24], rax
	add rsp, 8
	lea rdx, qword [rbp-8]
	mov rax, rdx
	push rax
	call _print_int_pointer
	add rsp, 8
	mov rdx, STR1
	push rdx
	call _print
	add rsp, 8
	mov rax, qword [rbp-16]
	push rax ; recalled `y`
	call _print_int_pointer
	add rsp, 8
	mov rdx, 10
	push rdx
	call _print_char
	add rsp, 8
	mov rax, qword [rbp-16]
	push rax ; recalled `y`
	pop rax
	sub rsp, 8
	mov rax, qword [rax+0]
	mov qword [rsp+0], rax
	push rax
	call _print_int
	add rsp, 8
	mov rdx, STR2
	push rdx
	call _print
	add rsp, 8
	mov rax, qword [rbp-24]
	push rax ; recalled `z`
	call _print_int
	add rsp, 8
	mov rdx, 10
	push rdx
	call _print_char
	add rsp, 8
	sub rsp, 16 ; stack reserved for `show`
	mov qword [rsp+0], 0
	mov qword [rsp+8], 0
	mov rdx, STR3
	push rdx
	lea rdx, qword [rbp-56]
	add rdx, 8 ; `name`
	mov r10, qword [rsp+0]
	mov qword [rdx+0], r10
	add rsp, 8 ; cleaned up stack
	mov rdx, 10
	push rdx
	lea rdx, qword [rbp-56]
	add rdx, 0 ; `rating`
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
	sub rsp, 8 ; stack reserved for `show_ptr`
	mov qword [rsp+0], 0
	lea rdx, qword [rbp-56]
	mov rax, rdx
	push rax
	mov rax, qword [rsp+0]
	mov qword [rbp-64], rax
	add rsp, 8
;TESTTESTTESTTEST
	mov rax, qword [rbp-64]
	push rax ; recalled `show_ptr`
;TESTESTESTESTESTESTESTESTEST
	pop rdx
	add rdx, 8 ; `name`
	sub rsp, 8 ; allocated space to push into
	mov r10, qword [rdx+0]
	mov qword [rsp+0], r10
	call _print
	add rsp, 8
	mov rdx, STR4
	push rdx
	call _print
	add rsp, 8
	lea rdx, qword [rbp-56]
	add rdx, 0 ; `rating`
	sub rsp, 8 ; allocated space to push into
	mov rax, qword [rdx+0]
	mov qword [rsp+0], rax
	call _print_int
	add rsp, 8
	mov rdx, STR5
	push rdx
	call _println
	add rsp, 8
	sub rsp, 16 ; stack reserved for `show_copy`
	mov qword [rsp+0], 0
	mov qword [rsp+8], 0
	mov rax, qword [rbp-64]
	push rax ; recalled `show_ptr`
	pop rax
	sub rsp, 16
	mov rax, qword [rax+0]
	mov qword [rsp+0], rax
	mov r10, qword [rax+8]
	mov qword [rsp+8], r10
	push rax
	mov rax, qword [rsp+0]
	mov qword [rbp-88], rax
	mov r10, qword [rsp+8]
	mov qword [rbp-80], r10
	add rsp, 16
	mov rdx, 128
	push rdx
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
	STR0 db "Bro thought I was new", 0
	STR1 db " ?= ", 0
	STR2 db " ?= ", 0
	STR3 db "STEINS;GATE", 0
	STR4 db " was rated ", 0
	STR5 db " stars.", 0
	GLO0: resb 1048576
	GLO1: resb 8
