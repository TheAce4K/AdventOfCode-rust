#![warn(clippy::pedantic)]

use std::fs::read_to_string;


fn main() {
    let filename = "input.txt";
    let data: &str = &read_to_string(filename).expect("File not found");
    let mut stacks1: Vec<Vec<&str>> = vec![vec![]; 9];

    for line in data.lines().take_while(|line| !line.contains("move")) {
        line.match_indices(char::is_uppercase).for_each(|(i, c)| {
            stacks1[(i-1)/4].insert(0, c);
        });
    }

    let mut stacks2: Vec<Vec<&str>> = stacks1.clone();

    for line in data.lines().skip_while(|line| !line.contains("move")) {
        let instructions: Vec<usize> = line.split_ascii_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect();
        for _i in 0..instructions[0] {
            let val = stacks1[instructions[1]-1].pop().expect("stack empty");
            stacks1[instructions[2]-1].push(val);
        }

        let stack_length = stacks2[instructions[1]-1].len();
        let mut vals: Vec<&str> = stacks2[instructions[1]-1].drain(stack_length-instructions[0]..).collect();
        stacks2[instructions[2]-1].append(&mut vals);
    }

    println!("Top values part 1: {:?}", stacks1.iter().map(|stack| stack.last().expect("stack empty")).collect::<Vec<&&str>>());

    println!("Top values part 2: {:?}", stacks2.iter().map(|stack| stack.last().expect("stack empty")).collect::<Vec<&&str>>());
}
