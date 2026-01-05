# Lisp Interpreter in Rust

MAL implementation in Rust in library form.

See [https://github.com/kanaka/mal](https://github.com/kanaka/mal) for detailed guidelines.

## Added Features

From the original MAL implementation, the following features have been added:

- std & no_std support.
- Support for floating point numbers in addition to integers.
- Support of streaming Lisp code input via Rust's `BufRead` trait.