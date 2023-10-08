# Defining Modules to Control Scope and Privacy

This module will address modules and paths, which comprise some aspects
for the module system.
The `use` keyword brings a *path* into scope; and, the `pub` keyword
makes that path public.

## Module Cheat Sheet

* **Start from the crate root**: When compiling a crate, the compiler
  first looks in the crate root file (usually `src/lib.rs` for a library
  crate or `src/main.rs` for a binary crate) for code to compile.
* **Declaring modules**: In the crate root file, you can declare new
  modules - for a module `mod garden;` declared in the root file, the 
  compiler will search for the module's code by default 
  in the following files:
    
    * Inline - within the curly braces that replace the semicolon
      following the `mod garden` statement
    * In the file `src/garden.rs`
    * In the file `src/garden/mod.rs`

* **Declaring submodules**: In any other than the crate root, you can
  declare submodules - for a submoduile `mod vegetables;` declared
  in the `src/garden.rs`, the compiler will search by default for the 
  submodules code in the following files:
    
    * Inline, directly following the `mod vegetables`, within the curly
      braces substituting the semicolon
    * In the file src/garden/vegetables.rs
    * In the file src/garden/vegetables/mod.rs

* **Paths to code in modules**: Once a module is part of your crate,
  you can refer to code in that module from anywhere else in that same
  crate, as long as the privacy rules allow for it, using the path to
  the code - `crate::garden::vegetables::Asparagus`
* **Private VS Public**: Code within a module is private from its
  parent modules by default.
  To make a module public, declare it with `pub mod` rather than `mod`
* **The `use` keyword**: The `use` keyword creates shortcuts to items
  to reduce the repetition of long paths.
  In any scope that can refer to `create::garden::vegetables::Asparagus`,
  you can create a shortcut with `use crate::garden::vegetables::Asparagus;`
  Afterwhich, you may refer to `Asparagus` directly.

## Grouping Related Code in Modules


