use std::fs::read_to_string;


fn find_value(c: char) -> u32 {
    match c {
        c if c.is_uppercase() => 26+c.to_digit(36).unwrap()-9,
        c if c.is_lowercase() => c.to_digit(36).unwrap()-9,
        _ => 0,
    }
}

fn part1(input: &String) -> u32 {
    input.split_terminator("\n").map(|line| {
        let matched_char: char = line[0..line.len()].chars().find(|c| line[line.len()/2..].contains(*c)).unwrap();
        find_value(matched_char) 
    }).sum::<u32>()
}
fn part2(input: &String) -> u32 {
    let lines: Vec<&str> = input.split_terminator("\n").collect::<Vec<&str>>();
    let mut sum: u32 = 0;
    for line in 0..lines.len()/3 {
        let matched_char: char = lines[line*3].chars().find(|c| lines[line*3+1].contains(*c) && lines[line*3+2].contains(*c)).unwrap();
        sum += find_value(matched_char);
    }
    sum
}

fn main() {
    let data: String = read_to_string("input.txt").expect("Error reading file");
    println!("Sum: {}", part1(&data));
    println!("Sum badge: {}", part2(&data));
}
