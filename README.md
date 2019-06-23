```
██████╗ ██╗███████╗██████╗
██╔══██╗██║██╔════╝██╔══██╗
██████╔╝██║███████╗██████╔╝
██╔══██╗██║╚════██║██╔═══╝
██║  ██║██║███████║██║
╚═╝  ╚═╝╚═╝╚══════╝╚═╝ A Lisp dialect written in Rust
```
## Installation
In order to use this interpreter, you must have Rust and Cargo installed. These can be obtained here: https://www.rust-lang.org/tools/install

To build the project, just do:
```
git clone https://github.com/geostran/risp.git && \
cd risp && \
cargo build --release && \
ln -s target/release/risp risp
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

The language does not implement some features that lisps usually have, such as macros, quasiquotes and closures.

Type `(help)` for more information when inside the REPL.

## Known issues
- A missing module will not be reported unless loaded explicitly from the REPL.
- Module paths are relative to the executable and not the files that load the modules.
