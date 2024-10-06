[X] Definitions
    [X] Maybe this will extend into some form of macro system?
     - Ex: `def PI 3.141592653f
            let x: float = 3 * PI;`

[-] Enums

[X] Readme file explaining how to at least compile a program

[ ] Add ability to move structs in memory
    [-] Should be able to move not just structs, but any data type

[-] Buff out the Command Line Interface because it needs it and is necessary
    - "./ignis -o main main.is"

[ ] Use enum variants for asm abstraction instead of strings - Ex:
             "\tadd rsp, 8" -> ASM::Add("rsp", "8")
             - IDK if this is actually any better, but it seems interesting.

[ ] Type casting
             [ ] Format: <type> value;
             [ ] Format: <!type> value;
             - Ex:
                          - `let a: int = <int> '\n'`
                          - converts character '\n' to int 10.
             - Ex:
                          - `let x: float = 4.0;
                             let b: int = <!int> x`
                          - Only changes type, not binary.
                          - converts floating point 4.0f to integer 1082130432 (yes actually).
                          - binary stays the same, so the value represented will change.
                          - any data that doesn't fit in the new value is dropped, preserving the end of the value.
             - Ex:
                          - `namespace TypeConv {
                                 func int_char :: (char, from: int) ...
                             }`
                          - Custom conversion methods can be defined in the TypeConv namespace
                             - Naming for conversion function is `[FROM]_[TO]`
                             - Note that the function MUST be in the TypeConv namespace, or else it is just a normal function
                             - TypePass is already defined for all conversions, in which nothing really happens to the data.



[X] Structs
             - Ex:
                          - `struct Person {
                                       name: []char;
                                       phone_number: int;
                             }`

[ ] Namespaces
             - Separator: '#'
             - Ex:
                          - `namespace cool_spot {
                                          func add :: (int, a: int, b: int) {
                                                       return a + b;
                                          }
                             }
                             let x: int = cool_spot#add(3, 4); // x = 7`

[X] Macros

[X] Variables should use rbp, not rsp
             - Ex:
                          - `sub rsp, 8
                             mov qword [rbp-8], 123`

[X] Finish implementing all operations
             [X] Still need a dereference operator (maybe use '@' sign for consistency?).
