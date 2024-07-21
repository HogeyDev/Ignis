[ ] Add ability to move structs in memory
    [ ] Should be able to move not just structs, but any data type

[-] Buff out the Command Line Interface because it needs it and is necessary
    - "./ignis -o main main.is"

[ ] Use enum variants for asm abstraction instead of strings - Ex:
             "\tadd rsp, 8" -> ASM::Add("rsp", "8")
             - IDK if this is actually any better, but it seems interesting.

[ ] Type casting
             [ ] Format: <type> value;
             - Ex:
                          - `let a: int = <int> '\n'`
                          - converts character '\n' to int 10.
             - Ex:
                          - `let x: float = 4.0;
                             let b: int = <!int> x`
                          - Only changes type, not binary.
                          - converts float 4.0 to int 1082130432 (yes actually).
                          - binary stays the same, so the value represented will change.
             - Ex:


[-] Structs
             - Ex:
                          - `struct Person {
                                       name: []char;
                                       phone_number: int;
                             }`

[ ] Namespaces
             - Ex:
                          - `namespace cool_spot {
                                          fn add: int = (a: int, b: int) {
                                                       return a + b;
                                          }
                             }
                             let x: int = cool_spot::add(3, 4); // x = 7`

[X] Variables should use rbp, not rsp
             - Ex:
                          - `sub rsp, 8
                             mov qword [rbp-8], 123`

[X] Finish implementing all operations
             [X] Still need a dereference operator (maybe use '@' sign for consistency?).
