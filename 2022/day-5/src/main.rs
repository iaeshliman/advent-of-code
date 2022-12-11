use regex::Regex;

fn display_state(state: &[Vec<char>; 9]) {
    let mut output: String = String::new();

    // Add header
    output.push_str("--------------- State ---------------\n");

    // Iterate over each stack
    for (i, stack) in state.iter().enumerate() {
        // Add stack identifier
        output.push_str(&format!("|| {} ||", i + 1));

        // Iterate over each item in stack adding to output
        for item in stack {
            output.push_str(&format!(" |{}|", item));
        }

        // Add ending newline
        output.push_str("\n");
    }

    // Add footer
    output.push_str("-------------------------------------");

    // Print the display
    println!("{}", output);
}

fn parse_input(input: &str) -> ([Vec<char>; 9], Vec<(u32, usize, usize)>) {
    // Split input between state and instructions
    let inputs: Vec<&str> = input.split("\n\n").collect();

    // Parse each input
    let state: [Vec<char>; 9] = parse_state(inputs[0].lines().collect());
    let instructions: Vec<(u32, usize, usize)> = parse_instruction(inputs[1].lines().collect());

    return (state, instructions);
}

fn parse_state(input: Vec<&str>) -> [Vec<char>; 9] {
    println!("Parsing initial state");

    // Array of vectors to represent each stack
    let mut state: [Vec<char>; 9] = [
        Vec::<char>::new(),
        Vec::<char>::new(),
        Vec::<char>::new(),
        Vec::<char>::new(),
        Vec::<char>::new(),
        Vec::<char>::new(),
        Vec::<char>::new(),
        Vec::<char>::new(),
        Vec::<char>::new(),
    ];

    // Iterate over each line
    // Reverse order to ensure stacks are added to in correct order
    // Skip first line as it only contains stack labels
    for line in input.iter().rev().skip(1) {
        // Iterate over each character in the line
        for (i, char) in line.chars().enumerate() {
            // Add item to stack when marker is found
            if char == '[' {
                // Calculate stack from the index of the marker
                let stack_index: usize = i as usize / 4;

                // Push next char onto the stack
                state[stack_index].push(line.chars().nth(i + 1).unwrap());
            }
        }
    }

    return state;
}

fn parse_instruction(input: Vec<&str>) -> Vec<(u32, usize, usize)> {
    println!("Parsing instructions");

    // Array of tuples to represent each instruction
    let mut instructions: Vec<(u32, usize, usize)> = Vec::new();

    // Regex pattern to match instruction against
    let re: Regex =
        Regex::new(r"^move (?P<count>[0-9]+) from (?P<src>[0-9]+) to (?P<dst>[0-9]+)$").unwrap();

    // Iterate over each line
    for line in input.iter() {
        // Compare line against regex to extract capture groups
        let captures = re.captures(line).unwrap();

        // Extract capture groups and convert to integers
        let count: u32 = captures.name("count").unwrap().as_str().parse().unwrap();
        let src: usize = captures.name("src").unwrap().as_str().parse().unwrap();
        let dst: usize = captures.name("dst").unwrap().as_str().parse().unwrap();

        // Add instruction to vector
        instructions.push((count, src, dst));
    }

    return instructions;
}

fn perform_instructions(
    state: &mut [Vec<char>; 9],
    instructions: &Vec<(u32, usize, usize)>,
    use_buffer: bool,
) {
    println!("Performing Instructions");

    // Iterate over instructions performing operation
    for instruction in instructions {
        let mut buffer: Vec<char> = Vec::new();

        for _ in 0..instruction.0 {
            // Borrow a reference to the source stack
            let src_stack: &mut Vec<char> = &mut state[instruction.1 - 1];

            // Pop the item off of the source stack
            let item: char = src_stack.pop().unwrap();

            // Add items to buffer
            if use_buffer {
                buffer.push(item);
            } else {
                // Borrow a reference to the destination stack
                let dst_stack: &mut Vec<char> = &mut state[instruction.2 - 1];

                // Add the item to destination stack
                dst_stack.push(item);
            }
        }

        if use_buffer {
            while !buffer.is_empty() {
                // Borrow a reference to the destination stack
                let dst_stack: &mut Vec<char> = &mut state[instruction.2 - 1];

                // Add the item from buffer to destination stack
                dst_stack.push(buffer.pop().unwrap());
            }
        }
    }
}

fn get_top_items(state: &[Vec<char>; 9]) -> Vec<char> {
    let mut items: Vec<char> = Vec::new();

    for stack in state {
        let item: char;

        // If the stack is empty continue to next
        match stack.last() {
            Some(x) => item = *x,
            None => continue,
        }

        // Add the top item to the list of items
        items.push(item);
    }

    return items;
}

fn puzzle_1(input: &str) {
    println!("=== Puzzle 1 ===");

    // Parse input
    let (mut state, instructions) = parse_input(input);

    display_state(&state);

    // Perform instructions
    perform_instructions(&mut state, &instructions, false);

    display_state(&state);

    // Evaluate top items
    let top_items: Vec<char> = get_top_items(&state);

    println!("Top items: {}", top_items.iter().collect::<String>());
}

fn puzzle_2(input: &str) {
    println!("=== Puzzle 2 ===");

    // Parse input
    let (mut state, instructions) = parse_input(input);

    display_state(&state);

    // Perform instructions
    perform_instructions(&mut state, &instructions, true);

    display_state(&state);

    // Evaluate top items
    let top_items: Vec<char> = get_top_items(&state);

    println!("Top items: {}", top_items.iter().collect::<String>());
}

fn main() {
    println!("===== Advent of Code - Day 5 =====");
    let input = include_str!("../data.txt").trim();
    puzzle_1(input);
    puzzle_2(input);
}
