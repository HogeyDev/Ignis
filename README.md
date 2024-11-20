# Ignis

> *Note that Ignis currently only develops and supports Linux, hopefully with support for Windows soon, and is currently only tested on ArchLinux at that*

### Dependencies
 - rust
 - gcc (only required for example/test.c)
 - nasm
 - ld

### Compiling the Ignis compiler
 - In the root project directory, simply run
    ```bash
    cargo build
    ```

### Compiling an Ignis program
 -  ```bash
    ./target/debug/ignis -o output input.is
    ```
    > *Note that the default location Rust outputs the binary would be `./target/debug/ignis`*
 - The compiler itself supports a few debugging flags
    - `--debug-tokens`
    - `--debug-ast`
    - `--debug-asm`

### Testing
 - Currently, tests are written in the small C file `example/test.c`
 - To add a file to debug:
    1. It must be located in the `example` directory
    2. In the main function, append 
       > *Note that the third argument should be left as an empty string*
    ```c
    addTest(&tests, "[TEST_NAME]", "", [EXPECTED_RETURN_VALUE]);
    ```
     - Ex:
        - We want to test: `example/my_cool_new_test.is`, which exits with the code `42`
        ```c
        addTest(&tests, "my_cool_new_test", "", 42);
        ```

### Versioning
 - Currently the scheme is MAJOR.MINOR.BUGFIX
    - Ex: `1.8.5` means that there is one major feature release, eight minor feature updates, and five bugfixes
