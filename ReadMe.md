# Topics Covered in this Section

## Packages, crates, and modules

1. Binary crates vs library crates
2. Submodules
3. The pub Keyword
4. Inline modules
5. File-based modules
6. Folder-based modules
7. Public enums, public structs, and public fields
8. The crate Prefix
9. The use Keyword
10. The self Keyword
11. The super Keyword
12. The as Keyword
13. The standard library
14. The Glob Operator (*)
15. Multiple binary crates
16. Documentation comments


### Crate
A crate is a collection of Rust code that produces an executable or a library.
A crate is the smallest amount of code that the Rust compiler considers at a time.

A binary crate is one that compiles to a an executable file `(has a main function)`.
A Library crate `( has no main function)` provides useful code to be executed by a binary crate.

### Package
A package is a collection of one or more crates.


The `pub` keyword makes sure that the element is accessible outside its namespace (scope).