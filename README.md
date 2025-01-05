# Unexpected Behavior with Immutable and Mutable References in Rust

This repository demonstrates a potential source of confusion in Rust regarding mutable and immutable references.  While Rust's borrow checker prevents certain types of data races, it's important to understand how mutable references can affect even seemingly immutable ones.

The `bug.rs` file shows a scenario where modifying a value through a mutable reference impacts a seemingly immutable reference. The `bugSolution.rs` file addresses the issue with an explanation and a suggestion to avoid such scenarios.