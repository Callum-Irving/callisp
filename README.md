# Callum's Lisp

![Build](https://img.shields.io/github/actions/workflow/status/Callum-Irving/callisp/rust.yml?style=flat-square)
[![Crates.io](https://img.shields.io/crates/v/callisp?style=flat-square)](https://crates.io/crates/callisp)
[![Docs](https://img.shields.io/docsrs/callisp?style=flat-square)](https://docs.rs/callisp/latest/callisp/)
![Files](https://img.shields.io/github/directory-file-count/Callum-Irving/callisp/src?style=flat-square)
![License](https://img.shields.io/crates/l/callisp?style=flat-square)

A Lisp interpreter designed to be run in the browser using WASM.

## Usage

You can perform simple numerical operations using +, -, *, and /:

```scheme
(+ 1 2) => 3
(/ 5 2) => 2.5
(- 2) => -2
(/ 5) => 0.2
```

You can define constants using `def`:

```scheme
(def x 3) => 3
x => 3
```

You can create functions using `lambda`:

```scheme
(lambda (x) (+ x 1)) => <function>
(def add1 (lambda (x) (+ x 1))) => <unspecified>
(add1 3) => 4
```

## List of all builtin functions and special forms

### Special forms

- `(def name value)`: creates a binding of name to value in current environment
- `(lambda (bindings) expr)` or `(λ (bindings) expr)`: creates a function
- `(if cond do else)`: evaluate do expr if cond is true, otherwise evaluate else expr

### Builtin functions

- `+`,`-`,`*`,`/`: simple arithmetic operators
- `exit`: exits with code 0 or code provided by argument
- `eval`: evaluate the expression passed as an argument
- `use`: evaluate all expressions contained in a file in the current environment
- `putstr`: print a string to stdout
- `readline`: read a line from stdin
- `equal?`: check if any amount of values are equal
- `>`, `>=`, `<`, `<=`: number comparison operators
- `list`: creates a list out of arguments given
- `list?`: returns true if argument is a list, otherwise returns false
- `empty?`: returns true if argument is a list of length 0, otherwise returns false
- `count`: returns the length of the argument if the argument is a list

## Goals

Goals in order of priority:

- [ ] Tail call optimization
- [ ] Macros
- [ ] Proper error handling
- [ ] Strings
- [ ] Multi-precision numbers
- [ ] Vectors
- [ ] Structs/enums/union types
- [x] IO (print, readline, etc.)
