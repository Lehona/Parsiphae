# Parsiphae

Parsiphae is an experimental Daedalus parser implemented in Rust. Currently it only supports syntax checking, but more features are coming soon(tm).

# Usage
After downloading or building Parsiphae, call `parsiphae.exe --src "path\to\Gothic.src"`.


# TODO
* Make sure that errors in typechecking are recoverable where possible.
* String literals that are used incorrectly in expressions fail to parse, which leads to bad error messages.
* Typechecking compares zPAR_TYPE, which does case-sensitive comparisons of non-primitive types
* Typechecking ignores array access in inst.member[X]
* Symbols should have an associated file_id, otherwise span information is pointless.
* Typechecking in instances has to consider member variable lookups
* Implement a fuzzy lookup mechanism to improve error messages.
* Redefined identifier not caught atm