# Advent of Code 2023 in Rust

This repository features Rust solutions for the [AOC 2023](https://adventofcode.com/2023/) code puzzles. Solutions are not optimal!

## Usage

### How to set up your own puzzle input

Each day has it's own directory. Solutions are located in `{day}/src/bin`. One sample input for each day is provided by default. To include your own personal input, do `touch {day}/src/my_input.txt` and paste in your puzzle input. Then replace the path inside the `include_str!` macro in any given solution: `include_str!("../my_input.txt")`.

### How to run a solution

`cd` into the directory of the day you want to run and do `cargo run --bin part1` to run part 1. The solution will be printed to STDOUT.
