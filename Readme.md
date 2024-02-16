# Parsiphae

Parsiphae is an experimental Daedalus parser implemented in Rust. Currently it only supports syntax checking, but more features are coming soon(tm).

# Usage
After downloading or building Parsiphae, call `parsiphae.exe --src "path\to\Gothic.src"`.


# TODO
* Make sure that errors in typechecking are recoverable where possible.
* Remove unwrap in main
* String literals that are used incorrectly in expressions fail to parse, which leads to bad error messages.
* Typechecking compares zPAR_TYPE, which does case-sensitive comparisons of non-primitive types