use std::fs;

fn main() {
    println!("Advent of Code - Day 1");

    // Get the contents of the file
    let contents: String = read_file("data.txt");

    // Count each groups totals
    let mut totals = parse_data(contents);

    // Find largest total
    let largest = find_max(&totals);

    // Print max
    println!("The most calories carried by a single elf is {}", largest);

    // Sort the data
    totals.sort();
    totals.reverse();

    println!(
        "The three largest calorie counts are {}, {}, and {}",
        totals[0], totals[1], totals[2]
    );

    println!(
        "This provides a total of {}",
        totals[0] + totals[1] + totals[2]
    );
}

// Reads file from passed path and returns contents as a string
fn read_file(path: &str) -> String {
    println!("reading file '{}'", path);

    return fs::read_to_string(path).expect("should have been able to read file");
}

fn parse_data(data: String) -> Vec<u32> {
    let mut totals: Vec<u32> = Vec::new();

    // Split data by each line
    let groups = data.split("\n\n");

    // Iterate over each line and
    for group in groups {
        let lines = group.split("\n");

        let mut total: u32 = 0;

        for line in lines {
            if line.is_empty() {
                continue;
            }
            match line.parse::<u32>() {
                Ok(n) => total += n,
                Err(e) => panic!("{}", e),
            }
        }

        totals.push(total);
    }

    return totals;
}

fn find_max(totals: &Vec<u32>) -> u32 {
    let mut max: u32 = 0;

    for total in totals {
        if *total > max {
            max = *total;
        }
    }

    return max;
}
