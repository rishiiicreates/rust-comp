# rust-comp (writing a rust compiler from scratch because sanity was never an option T_T)

welcome to my brain dump and ongoing struggle of building a rust compiler completely from scratch. yes, compiling rust using rust. fighting rustc's borrow checker while trying to build our own borrow checker is peak psychological damage, but we ball anyway (._.)

## what's actually inside this mess?
- **the lexer:** turning source code into tokens until an unclosed string literal sends the parser into an existential crisis :3
- **the parser & ast:** converting flat tokens into recursive syntax trees while wrapping literally everything in `Box<T>` because `error[E0072]: recursive type has infinite size` humbled me at 3 am.
- **type checking & semantics:** trying to enforce strict types and lifetime rules when i can barely enforce my own sleep schedule.
- **the borrow checker:** the final boss. currently standing 10 feet away from it and pretending it doesn't see me. if you see lifetime logic in here, please say a prayer for it.
- **codegen & backend:** spitting out bytecode / assembly and praying to the 16-byte stack alignment gods so macOS doesn't immediately segfault.

## current state of the chaos
- **lexer:** working and vibing with keywords, numbers, strings, and operators
- **parser:** parses basic let bindings, returns, and literals without catching fire
- **borrow checker:** emotional damage pending
- **codegen:** currently emitting pure hopes and prayers

# Note: This is not rustc. If rustc takes 3 seconds to compile, this thing might either finish in 2 milliseconds or summon an ancient demon that consumes all 16GB of unified memory behind my back.

## How to run (at your own risk)

```bash
# run the built-in demo snippet
cargo run

# pass your own file if you're feeling brave
cargo run -- path/to/test.rs

# run the test suite (they actually pass right now, nobody breathe)
cargo test
```

## Contributing
if you spot an unwrap() that makes your heart sink, or my AST nodes give you second-hand embarrassment, please open a PR. or just roast my terrible logic in the issues tab, i completely deserve it. virtual chai and good vibes guaranteed T_T
