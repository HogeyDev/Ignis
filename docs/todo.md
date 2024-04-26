[ ] Use enum variants for asm abstraction instead of strings
    - Ex:
        "\tadd rsp, 8" -> ASM::Add("rsp", "8")
    - IDK if this is actually any better, but it seems interesting

[ ] Add more types?

[ ] Finish implementing all operations
    - Still need a dereference operator (maybe use '@' sign for consistency?)
