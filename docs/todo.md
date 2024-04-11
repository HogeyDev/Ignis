[ ] Variables should use rbp, not rsp
    - Ex:
        sub rsp, 8
        mov qword [rbp-8], 123
