# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this repo is

Rust solutions to the LeetCode "Top Interview 150" study plan
(<https://leetcode.com/studyplan/top-interview-150/>). It is a practice repo, not
an application: `src/main.rs` is the stock hello-world stub and all real content
lives in the library crate. Rust edition 2024, no dependencies.

## Commands

```sh
cargo test                                   # run all solution tests
cargo test p88                               # run one problem's tests (filter matches the module path)
cargo test leet150::p88_merge_sorted_array::tests::example_1   # run a single test
cargo clippy --all-targets                   # lint
cargo fmt                                    # format
```

## Module layout

Use the 2018-and-later module style: a module's own code goes in `<name>.rs`
*beside* its directory of children, never in `<name>/mod.rs`. So `src/leet150.rs`
declares the submodules and `src/leet150/` holds them. Do not reintroduce
`mod.rs` files.

## Adding a solution

Each problem is one file, one module, self-contained:

1. Create `src/leet150/p<number>_<snake_case_title>.rs` (e.g. `p88_merge_sorted_array.rs`).
2. Register it in `src/leet150.rs` as `mod p<number>_<title>;` — keep the list
   in the order problems are solved, not alphabetical.
3. Head the solution function with a doc comment: `/// <number>. <Title>` followed
   by a restatement of the problem constraints in your own words.
4. Keep LeetCode's exact function signature, including its un-Rusty parts
   (`&mut Vec<i32>` instead of `&mut [i32]`, `i32` lengths, `nums1`/`nums2` naming),
   so the body can be pasted straight into the LeetCode editor. Do not "fix" these
   signatures.
5. Add a `#[cfg(test)] mod tests` with one `#[test]` per example from the problem
   statement, named `example_1`, `example_2`, … Add extra tests for edge cases
   beyond the examples.
6. Tick the problem's checkbox in `Top150.md`.

`Top150.md` is the progress tracker: all 150 problems grouped by the study plan's
topic sections (Array/String, Two Pointers, … Multidimensional DP), each as a
`- [ ] <number>. <Title> (<Difficulty>)` line.

## Known wart

Solution functions are only called from their own tests, so `cargo build` warns
`function ... is never used`. `p88` carries `#[warn(dead_code)]`, which is the
default level and therefore does not silence anything — `#[allow(dead_code)]` is
what suppresses it. Match whatever the surrounding files settle on rather than
introducing a third style.
