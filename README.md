# minigrep

## Overview

A example implementation of of Chapter 12 in *The Rust Programming Language (TRPL)*.

Grep is an acronym for "Globally search a Regular Expression and Print", which is a command-line utility for searching plain-text data sets for lines that match a regular expression.

## Some thoughts

### Best practices for packages with a binary and a library

A package can contain both a `src/main.rs` binary crate root as well as a `src/lib.rs` library crate root, and both crates will have the package name by default. Typically, packages with this pattern of containing both a library and a binary crate will have just enough code in the binary crate to start an executable that calls code within the library crate. This lets other projects benefit from most of the functionality that the package provides because the library crate’s code can be shared.

The module tree should be defined in `src/lib.rs`. Then, any public items can be used in the binary crate by starting paths with the name of the package. The binary crate becomes a user of the library crate just like a completely external crate would use the library crate: it can only use the public API. This helps you design a good API; not only are you the author, you’re also a client!

### Why we need to manually label the life cycle

Why do we need to manually label the lifecycle when the compiler can give a unique lifecycle labeling hint. For the time being, I have not found any cases of undecidable code, i.e., they have identical function body contents except for different life cycle annotations, and they all compile successfully.

I'm currently of the opinion that the return value of a function needs to be labeled with the same lifecycle as the function parameters involved, and this is a rule that I think the compiler can infer from the internal code of the function. So a simple understanding of the purpose of manual labeling is to facilitate the compiler can only know the function signature in the case of function parameters and return value external life cycle check.

Also, why the following code can be compiled successfully. According to the labeling of `'b:'a`, `y/string2` should live at least longer than `x/string1`, but in fact `string2` has a shorter life cycle than `string1`.

```rust
fn main() {
    let string1 = String::from("short string");

    {
        let string2 = String::from("longer string");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    println!("string1 is {}", string1);
}

fn longest<'a, 'b:'a>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
