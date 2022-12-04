#![warn(clippy::pedantic)]

use std::str::FromStr;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
// read whole data file to string
fn read_data(filename: &str) -> String {
    let file = File::open(filename).expect("File not found");
    let mut data = String::new();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut data).expect("Error reading file");
    return data;
}


enum Moves {
    R,
    P,
    S,
}


impl FromStr for Moves {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Moves, ()> {
        match s {
            "A" | "X" => Ok(Moves::R),
            "B" | "Y" => Ok(Moves::P),
            "C" | "Z" => Ok(Moves::S),
            _ => Err(()),
        }
    }
}


impl Moves {
    fn value(&self) -> i8 {
        match self {
            Moves::R => 0,
            Moves::P => 1,
            Moves::S => 2,
        }
    }
}


fn calculate_score_part_1(line: &str) -> u32 {
    let mut score: u32 = 0;
    println!("Line: {}", line);
    let result: Vec<Moves> = line.split(" ").map(|x| x.parse::<Moves>().unwrap()).collect();
    match (3+result[0].value()-result[1].value())%3 {
        0 => score += 3,
        1 => score += 0,
        2 => score += 6,
        _ => panic!("Error Not accepted value when determining winner"),
    }
    match result[1] {
        Moves::R => score += 1,
        Moves::P => score += 2,
        Moves::S => score += 3,
    }
    return score;
}


fn calculate_score_part_2(line: &str) -> u32 {
    let mut score: u32 = 0;
    println!("Line: {}", line);
    let result: Vec<&str> = line.split(" ").collect();
    match result[1] {
        "X" => score += 0,
        "Y" => score += 3,
        "Z" => score += 6,
        _ => panic!("Error Not accepted value when if you should win"),
    }
    let opponent_move_value: i8 = result[0].parse::<Moves>().unwrap().value();
    let my_move_val: i8;
    match result[1] {
        "X" => my_move_val = (3+opponent_move_value-1)%3,
        "Y" => my_move_val = opponent_move_value,
        "Z" => my_move_val = (opponent_move_value+1)%3,
        _ => panic!("Error Not accepted value when determining winner"),
    }
    match my_move_val {
        0 => score += 1,
        1 => score += 2,
        2 => score += 3,
        _ => panic!("Error Not accepted value when determining winner"),
    }
    return score;
}


fn main() {

    println!("Hello, world!");
    //println!("Score: {}",calculate_score("B Z"));
    let score1: u32 = read_data("data.txt").split_terminator("\n").map(|line| calculate_score_part_1(line)).sum();
    let score2: u32 = read_data("data.txt").split_terminator("\n").map(|line| calculate_score_part_2(line)).sum();
    println!("Sum of score 1: {}", score1);
    println!("Sum of score 2: {}", score2);
}
