# Rust Stuco S24 Initial Plan

Spring 2024 tentative syllabus

# Materials:
- Main Source: [The Rust Programming Langauge (The Rust Book)](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [The Rust Reference](https://doc.rust-lang.org/reference/index.html)
- Rust for Rustaceans - pdf on oreilly??
- Exercism??


# Schedule


## Week 1: Introduction

### Lecture
- Why Rust? (10 min)
- Cargo Basics (`new`, `build`, `run`, `check`)
- Syntax
  - Variables and Mutability
  - Data Types
    - Scalar types
    - Compound types
  - Functions
  - Control Flow

### Reading
- Rust Book Chapters 1-3
- Maybe a video about Rust?

### Assignment
- Guessing game plus
- 15-112 type problems



## Week 2: Ownership

### Lecture
- Ownership
  - Move semantics
  - Scope
  - Why Ownership?
    - `String` type basics
    - Memory and Allocation
  - Copy and Clone Traits
- References and Borrowing
  - Reference vs address of
  - Mutable references
  - Dangling references and data racing
  - Automatic referencing and dereferencing sneak peek
- Slices
  - Memory layout
  - `&str`

### Reading
- Rust Book Chapter 4
- Probably direct to videos online

### Assignment
- Move semantics
- String stuff (rustlings for inspiration?)
- Something else



## Week 3: Structs and Enums

### Lecture
- Structs
  - Creating Structs
    - Update syntax
  - Tuple Structs
  - Unit Structs
  - Lifetimes sneak peek
- `impl` Structs and Methods
  - Automatic referencing and dereferencing
  - Associated functions
- Enum basics
  - Pattern Matching
  - Option Type
  - `if let`

### Reading
Rust Book Chapters 5-6

### Assignment
- `struct` problems (15-122 type problems?)
- 15-150 type problems




## Week 4: Cargo, Testing, Modules, and Crates (TODO)
Chapters 7, 14, and 11 + maybe the Cargo Reference

- Cargo (TODO)
- File system
  - Packages
  - Crates
    - Binary vs Library crate
  - Modules
    - `mod` keyword
    - Scope, Privacy, and Paths
    - Module organization
- Testing
  - `#[cfg(test)]` and `#[test]`
  - `#[should_panic]` and `Result<T, E>` in tests
  - `cargo test` usage and flags
  - Unit testing





## Week 5: Standard Collections and Error Handling
Chapters 8-9

- Vectors
- Strings
  - UTF-8
  - char length
- Hash Maps
  - `Entry` API
- BTree
- Error Handling
  - `panic!()`
  - `Result<T, E>`
  - Error propogations with `?`



## Week 6: Generics and Traits (TODO)
Chapter 10.1-10.2 + probably more

- Generics basics
  - Metaprogramming
  - Structs, Enums, Function definitions
  - Monomorphization vs. Polymorphism
  - Performance
- Trait basics
- Derivable Traits
- OOP discussion (Chapter 17)?



## Week 7: Lifetimes (TODO)
Chapter 10.3 + probably need more

- Very important, need to get this right



## Week 8: Iterators, Closures, and Advanced Functional (TODO)
Chapter 13




## Week 9: Smart Pointers and Trait Objects (TODO)
Chapter 15

- `Box<T>`
- `drop()`
- `Rc<T>`
- `RefCell<T>`
  - `UnsafeCell<T>`
- Memory leaks
- Trait Objects
  - Wide pointers
  - Dynamic dispatch with `dyn`
  - Vtable
  - Probably need more



## Week 10: Fearless Concurrency (TODO)
Chapter 16



## Week 11: Advanced Rust (TODO)
Chapters 17-19

- Unsafe
  - FFI
- Macros



## Week 12: Final Project (TODO)

- API design
