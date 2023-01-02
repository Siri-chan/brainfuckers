# brainfucke-rs

A brainfuck processor in Rust.

## Implementation

Supports all 8 Brainfuck Commands, properly ignores all non-command characters, and uses a dynamically-allocated vector for the array, meaning that it is only limited by hardware (because `usize` and memory being fundamentally finite etc.). However, it does NOT extend leftward, nor does the pointer wrap around. The program will just panic.  
This specific distribution uses `usize` for an instruction pointer, `u8`s for each memory cell, and will print a full readout of memory in both an hexadecimal readout and in UTF8 bytes as a program ends. <!-- ? Is it really a good idea to print a full readout? maybe this should be an argument flag?? -->
This is in addition to the `.` and `,` operators, which will output while the program is running.  
If `,` recieves an EOF, the program will panic.  
The distribution will panic on integer over/underflows in any cell also.  
LF vs CRLF in operating systems should not matter, as the program ignores any non-command characters, and should output platform-agnostically thanks to rust's `println!` macro.  
BOptimisations are best made by the code author, so be careful to prune your code properly in order to ensure best performance.

## Usage

Use `brainfuckers <file>` to run the program in `<file>`, or just `brainfuckers` to read from `stdin` and run that. `brainfuckers -c <file>` to transpile the brainfuck code in `<file>` to C code.  <!-- The `-cC` argument can also be used to automatically run GCC on the generated c code. `-C` will panic on it's own however. -->

## Caveats with Transpilation

1. Transpilation is DUMB. This means that it literally just does pattern substitution on the brainfuck tokens, rather than doing anything cool with them.
2. Transpilation does **NOT** preserve comments.
3. Transpilation uses the **C equivalents** found on the Wikipedia Brainfuck article as of December 12, 2022, combined with some custom boilerplate.  
This means (along with being in C in the first place) that the once infinite data array is now only 30,000 cells long.
4. Every brainfuck character matches one line of C code.  
This may not seem like a downside, but given the length of Brainfuck programs, it often is.  
See why in the following code:

```bf
// Brainfuck Code: [-]
```

```c
// Auto-Generated C Code:
while (*ptr) {
    --*ptr;
}
// Human-Generated C Code:
while (--*ptr){}
```

## Still Unfinished

This project is nearly complete. The remaining goals are listed below:

- [ ] Use the `cc` crate to natively compile the auto-generated C code.
- [ ] Implement TTY Raw Mode (using `crossterm`, probably<!-- I prefer Termion but Windows Support is Important -->) so that the interpreter can properly execute `,` instructions.
- [ ] Seperate code into more organised modules to avoid the current spaghetti situation.
