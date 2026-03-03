# LeetCode in Rust

Solutions to LeetCode problems implemented in Rust, with multiple approaches per problem — from brute force to idiomatic Rust.

## Structure

```
src/
├── lib.rs
├── solutions.rs
└── solutions/
    └── s01_two_sum.rs
```

Each solution file contains multiple implementations showcasing different approaches and complexity trade-offs. Tests are included inline via `#[cfg(test)]`.

## Solutions

| #  | Problem  | Approaches |
|----|----------|------------|
| 01 | [Two Sum](https://leetcode.com/problems/two-sum/) | brute force O(n²), hash map O(n), idiomatic Rust |

## Running

```bash
cargo test                              # run all tests
cargo test s01                          # run tests for a specific problem
cargo test -- --nocapture               # run tests with stdout output
```
