use std::fs::read_to_string;

fn part1(input: &String) -> u32 {
    input.split_terminator("\n").map(|line| {
        match line.split(|c| c == ',' || c == '-').map(|bounds| {
            bounds.parse::<u32>().expect("cant convert to u32")
        }).collect::<Vec<u32>>() {
            line if line[0] <= line[2] && line[3] <= line[1] => 1,
            line if line[2] <= line[0] && line[1] <= line[3] => 1,
            _ => 0,
        }
    }).sum()
}


fn part2(input: &String) -> u32 {
    input.split_terminator("\n").map(|line| {
        match line.split(|c| c == ',' || c == '-').map(|bounds| {
            bounds.parse::<u32>().expect("cant convert to u32")
        }).collect::<Vec<u32>>() {
            line if line[0] <= line[3] && line[0] >= line[2] => 1,
            line if line[1] <= line[3] && line[1] >= line[2] => 1,
            line if line[2] <= line[1] && line[2] >= line[0] => 1,
            line if line[3] <= line[1] && line[3] >= line[0] => 1,
            _ => 0,
        }
    }).sum()
}


fn main() {
    println!("Hello, world!");
    let filename = "data.txt";
    let data: String = read_to_string(filename).expect("File not found");
    println!("Sum of full overlap: {}", part1(&data));
    println!("Sum of overlap: {}", part2(&data));
    
}
