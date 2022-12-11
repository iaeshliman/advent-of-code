use std::collections::{HashMap, HashSet};

fn find_marker(stream: &str, window_size: usize) -> usize {
    return stream
        .as_bytes()
        .windows(window_size)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == window_size)
        .unwrap()
        + window_size;
}

fn puzzle_1(input: &str) {
    println!("=== Puzzle 1 ===");

    // Find index of first character after marker
    let index: usize = find_marker(input, 4);

    println!("Index after start-of-packet marker: {}", index);
}

fn puzzle_2(input: &str) {
    println!("=== Puzzle 2 ===");

    // Find index of first character after marker
    let index: usize = find_marker(input, 14);

    println!("Index after start-of-message marker: {}", index);
}

fn main() {
    println!("===== Advent of Code - Day 5 =====");
    let input = include_str!("../data.txt").trim();
    puzzle_1(input);
    puzzle_2(input);
}
