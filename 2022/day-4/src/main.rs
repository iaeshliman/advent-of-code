fn parse_input(input: &str) -> Vec<Vec<(u32, u32)>> {
    let mut groups: Vec<Vec<(u32, u32)>> = Vec::new();

    // Iterate over each line
    for line in input.lines() {
        // Split line into pairs
        let group: Vec<(u32, u32)> = line
            .split(',')
            .map(
                |x: &str| match x.split("-").collect::<Vec<&str>>().chunks(2).next() {
                    Some(value) => {
                        return (
                            value[0].parse::<u32>().unwrap(),
                            value[1].parse::<u32>().unwrap(),
                        )
                    }
                    None => panic!("should have found a chunk"),
                },
            )
            .collect();
        groups.push(group);
    }

    return groups;
}

//
// Elf A        Elf B       Difference  Contained   Overlapped
// (12, 15)     (10, 14)    (2, 1)      false       true
// (12, 15)     (10, 16)    (2, -1)     true        true
// (12, 15)     (12, 16)    (0, -1)     true        true
// (12, 15)     (13, 14)    (-1, 1)     true        true
// (12, 15)     (16, 20)    (-4, -5)    false       false
// (12, 15)     (8, 10)     (4, 5)      false       false
fn is_contained(pair_a: (u32, u32), pair_b: (u32, u32)) -> bool {
    let lower: i32 = pair_a.0 as i32 - pair_b.0 as i32;
    let upper: i32 = pair_a.1 as i32 - pair_b.1 as i32;
    return lower * upper <= 0;
}

fn is_overlapped(pair_a: (u32, u32), pair_b: (u32, u32)) -> bool {
    if pair_a.0 > pair_b.1 {
        return false;
    }
    if pair_a.1 < pair_b.0 {
        return false;
    }
    return true;
}

enum Coverage {
    Total,
    Partial,
    None,
}

fn evaluate_group(group: &Vec<(u32, u32)>) -> Coverage {
    for i in 0..(group.len() - 1) {
        if is_contained(group[i], group[i + 1]) {
            return Coverage::Total;
        }
        if is_overlapped(group[i], group[i + 1]) {
            return Coverage::Partial;
        }
    }
    return Coverage::None;
}

fn puzzle_1(input: &str) {
    println!("===== Solving puzzle 1 =====");

    // Parse input into groups
    let groups: Vec<Vec<(u32, u32)>> = parse_input(input);

    // Evaluate each group for assignment containment
    let mut count: u32 = 0;

    // Iterate over each group evaluating containment
    for group in &groups {
        match evaluate_group(group) {
            Coverage::Total => count += 1,
            _ => continue,
        }
    }

    println!(
        "Total number of groups with complete assignment containment: {}",
        count
    );
}

fn puzzle_2(input: &str) {
    println!("===== Solving puzzle 2 =====");

    // Parse input into groups
    let groups: Vec<Vec<(u32, u32)>> = parse_input(input);

    // Evaluate each group for assignment containment
    let mut count: u32 = 0;

    // Iterate over each group evaluating containment
    for group in &groups {
        match evaluate_group(group) {
            Coverage::Total => count += 1,
            Coverage::Partial => count += 1,
            _ => continue,
        }
    }

    println!(
        "Total number of groups with partial assignment containment: {}",
        count
    );
}

fn main() {
    println!("===== Advent of Code - Day 4 =====");
    let input = include_str!("../data.txt").trim();
    puzzle_1(input);
    puzzle_2(input);
}
