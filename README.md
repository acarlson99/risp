# Risp
A Lisp dialect written in Rust
## Installation
In order to use this interpreter, you must have Rust and Cargo installed. These can be obtained here: https://www.rust-lang.org/tools/install

To build the project, just do:
```
git clone https://github.com/geostran/risp.git
cd risp
cargo build --release
ln -s risp target/release/risp
```

After it's built, you can run it as:
`./risp` or `./risp filename`
## Features
- Erros as first class values.
- Strings, Symbols, Floats and Integers.
- Built-in support for vectors and hashmaps.
- Quotes and lambdas.
- Module system.
- Lexical scoping.
- Mutual recursion.

The language is Turing-complete, but it does not implement all the features that lisps usually have. For example, it lacks macros, quasiquotes and closures.
It does have normal quotes and lambdas.

Type `(help)` for more information when inside the REPL.

## Known issues
- A missing module will not be reported unless loaded explicitly from the REPL.
