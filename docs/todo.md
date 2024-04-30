[ ] Use enum variants for asm abstraction instead of strings
    - Ex:
        "\tadd rsp, 8" -> ASM::Add("rsp", "8")
    - IDK if this is actually any better, but it seems interesting.

[ ] Type casting
    - Format: <type> value;
    - Ex:
        let a: int = <int> '\n'
        - converts character '\n' to int 10.
    - Ex:
        let x: float = 4.0;
        let b: int = <!int> x
        - Only changes type, not binary.
        - converts float 4.0 to int 1082130432 (yes actually).
        - binary stays the same, so the value represented will change.

[ ] Structs
    - Ex:
        struct Person {
            name: []char;
            phone_number: int;
        }

[X] Variables should use rbp, not rsp
    - Ex:
        sub rsp, 8
        mov qword [rbp-8], 123

[X] Finish implementing all operations
    - Still need a dereference operator (maybe use '@' sign for consistency?).
