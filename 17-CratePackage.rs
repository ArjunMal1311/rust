// A crate can contain one or more Rust modules which in turn contain code such as functions

// A binary crate is a Rust program that compiles to an executable or multiple executables and has a main() function for each executable.
// A library crate doesn't compile to an executable and doesn't have a main() function. A library crate generally defines a shared functionality that can be used in multiple projects.

// Rust Cargo
// Cargo is Rust package manager. Comes preinstalled with rust and can be used to install
// package dependecies and manage them as well as build and distribute own package/libraries

// Features
// 1. Dependency Management
// 2. Building and Packaging
// 3. Document Generation
// 4. Downloading crates
// 5. Run a binary or tests


// cargo new [package]

// Build the project --> cargo build

// We can also add dependency to the project using cargo add rand
// Command	Description
// cargo new	Create a new Rust project with basic directory structure
// cargo build	Build (compile) the current project and generate a binary executable
// cargo run	Build and run your current project (cargo build + run)
// cargo check	Build the current project without generating a binary executable
// cargo add	Add a new dependency and include it in Cargo.toml file
// cargo update	Update all dependencies of current project to latest version