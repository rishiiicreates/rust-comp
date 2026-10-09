# rust-comp (building a python compiler in rust from scratch because cpython wasn't fast enough T_T)

welcome to my brain dump of building a python compiler from scratch using rust. yes, taking the friendliest, most dynamic scripting language in existence and compiling it with the most aggressive, borrow-checking systems language known to humanity. it is going exactly as chaotic as you would imagine (._.)

## what's actually happening in this repo
- **the indentation scanner:** python does not have cute curly braces. it has tabs, spaces, and vibes. our lexer has to manage an indent and dedent stack without having an emotional breakdown :3
- **the parser & ast:** turning python statements, if blocks, while loops, and def function declarations into recursive syntax trees wrapped in `Box<T>` because rustc does not negotiate with infinite size types.
- **the dynamic type system:** python variables can be an integer on line one and a string on line two. implementing dynamic pyvalue enums in rust while rustc screams for static types is peak comedy.
- **the bytecode vm & backend:** compiling the AST down to clean bytecode instructions and running them on our own stack-based virtual machine.

## current status of the chaos
- **lexer & indent stack:** getting designed right now
- **parser:** warming up brain cells
- **runtime & pyvalues:** preparing for dynamic type madness
- **bytecode vm:** emitting pure hopes and prayers

# Note: This is not CPython. If your python script crashes here, there is a solid chance my indent stack miscalculated two spaces at 3 AM.

## How to run (at your own risk)

```bash
# run the compiler
cargo run

# run tests
cargo test
```

## Contributing
if you spot a bug in my indent scanner, or my AST nodes give you second-hand embarrassment, please open a PR. or roast my terrible logic in the issues tab, i completely deserve it. virtual chai and good vibes guaranteed T_T
