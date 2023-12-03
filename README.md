# Advent of Code 2023 in Rust

This repository features Rust solutions for the [AOC 2023](https://adventofcode.com/2023/) code puzzles. Solutions are not optimal!

## How to use

Each day has it's own directory. Solutions are located in `{day}/src/bin`. One sample input for each day is provided by default. To run your own personal input, do `touch {day}/src/my_input.txt` and paste in your puzzle input. Then replace the path inside the `include_str!` macro in any given solution: `include_str!("../my_input.txt")`.
