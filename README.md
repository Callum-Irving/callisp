# Callum's Lisp

![Build](https://img.shields.io/github/actions/workflow/status/Callum-Irving/callisp/rust.yml?style=for-the-badge)
[![Crates.io](https://img.shields.io/crates/v/callisp?style=for-the-badge)](https://crates.io/crates/callisp)
[![Docs](https://img.shields.io/docsrs/callisp?style=for-the-badge)](https://docs.rs/callisp/latest/callisp/)
![Lines of code](https://img.shields.io/tokei/lines/github/Callum-Irving/callisp?style=for-the-badge)
![Files](https://img.shields.io/github/directory-file-count/Callum-Irving/callisp/src?style=for-the-badge)

A Lisp interpreter designed to be run in the browser using WASM.

## Usage

You can perform simple numerical operations using +, -, *, and /:

```scheme
(+ 1 2) => 3
(/ 5 2) => 2.5
(- 2) => -2
(/ 5) => 0.2
```

You can define constants using `define`:

```scheme
(define x 3) => 3
x => 3
```

Define also returns the value of the constant that was defined.

You can create functions using `lambda`:

```scheme
(lambda (x) (+ x 1)) => <function>
(define add1 (lambda (x) (+ x 1))) => <function>
(add1 3) => 4
```

## List of all builtin functions and special forms

### Special forms

- `define`: creates a constant in the current environment
- `lambda`: creates a function

### Builtin functions

- `exit`: exits with code 0 or code provided by argument
- `+`,`-`,`*`,`/`: simple arithmetic operators

## Goals

- [ ] IO (print, readline, etc.)
- [ ] Macros
- [ ] Tail call elimination
- [ ] Multi-precision numbers
- [ ] Strings
- [ ] Vectors
