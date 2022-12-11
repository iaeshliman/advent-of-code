/**
 * Dependencies
 */
use std::collections::HashSet;

/**
 * Helper Functions
 */

//  Puzzle 1
fn parse_line(line: &str) -> (&str, &str) {
    // Split line into two equal parts
    let group_1: &str = &line[..line.len() / 2];
    let group_2: &str = &line[line.len() / 2..];

    return (group_1, group_2);
}

fn find_duplicates(group_1: &str, group_2: &str) -> HashSet<char> {
    let mut duplicates: HashSet<char> = HashSet::new();

    // Iterate over each group and compare for any duplicates
    for i in group_1.chars() {
        for j in group_2.chars() {
            // Add char to set if it exists in both groups
            if i == j {
                duplicates.insert(i);
            }
        }
    }

    return duplicates;
}

fn map_priority(element: char) -> u32 {
    match element {
        'A'..='Z' => return element as u32 - 38,
        'a'..='z' => return element as u32 - 96,
        _ => panic!("element '{}' should be a letter", element),
    }
}

// Puzzle 2

/// Returns the character all three series have in common
/// Assumes each series does have one and only one common character
fn find_common_char(group: &[&str]) -> char {
    // Extract each series
    let series_a: &str = group[0];
    let series_b: &str = group[1];
    let series_c: &str = group[2];

    // Iterate over each series looking for a common character
    for char_a in series_a.chars() {
        for char_b in series_b.chars() {
            for char_c in series_c.chars() {
                if char_a == char_b && char_a == char_c {
                    return char_a;
                }
            }
        }
    }

    panic!("should have found a common char");
}

/**
 * Control Flow
 */
fn part_1(data: &str) {
    println!("Puzzle part 1 goes here");

    // Sum of all priorities
    let mut total: u32 = 0;

    // Iterate over each line
    for line in data.lines() {
        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        // Split line into each group
        let (group_1, group_2) = parse_line(line);

        // Find each duplicate among the groups
        let duplicates = find_duplicates(group_1, group_2);

        // Sum of this lines duplicates
        let mut sum: u32 = 0;

        // If duplicates exist find the sum of their priorities
        if duplicates.len() > 0 {
            // Iterate over each duplicate summing the priority
            for duplicate in duplicates {
                sum += map_priority(duplicate);
            }
        }

        total += sum;
    }

    println!("Sum of all priorities: {}", total);
}

fn part_2(data: &str) {
    println!("Puzzle part 2 goes here");

    const GROUP_SIZE: usize = 3;
    let mut total: u32 = 0;

    // Convert data into an iterator over each line and iterate over chunks
    for chunk in data.lines().collect::<Vec<&str>>().chunks(GROUP_SIZE) {
        // Extract the common char in the chunk
        let common_char: char = find_common_char(chunk);

        total += map_priority(common_char);
    }

    println!("Total priority: {}", total);
}

fn main() {
    println!("Advent of Code - Day 3");
    let data: &str = include_str!("../data.txt").trim();
    part_1(data);
    part_2(data);
}
