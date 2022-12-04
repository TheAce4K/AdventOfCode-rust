use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


fn read_data (filename: &str) -> String {
    let file = File::open(filename).expect("File not found");
    let mut data = String::new();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut data).expect("Error reading file");
    return data;
}


fn main() {
    println!("Hello, world!");
    let data = read_data("input.txt");
    let mut matched_chars: Vec<u32> = Vec::new();
    data.split_terminator("\n").for_each( |line| {
        println!("Line: {}", line);
        println!("Line length: {}", line.len());
        println!("first part: {}", &line[0..(line.len()/2)]);
        println!("second part: {}", &line[(line.len()/2)..]);
        for c in line[0..line.len()/2].chars() {
            if line[line.len()/2..].find(c).is_some() {
                println!("Found match: {}", c);
                let char_value: u32; 
                if c.is_uppercase() {
                    char_value = 26+(c.to_digit(36).unwrap()-9);
                } else {
                    char_value = (c.to_digit(36).unwrap()-9).try_into().unwrap();
                }
                println!("Value: {}", char_value);
                matched_chars.push(char_value);
                break;
            }
        };
    });
    let lines: Vec<&str> = data.split_terminator("\n").collect();
    let mut matched_badges: Vec<u32> = Vec::new();
    for i in 0..lines.len()/3 {
        for c in lines[(i)*3].chars() {
            if lines[(i)*3+1].find(c).is_some() {
                if lines[(i)*3+2].find(c).is_some() {
                    let char_value: u32;
                    if c.is_uppercase() {
                        char_value = 26+(c.to_digit(36).unwrap()-9);
                    } else {
                        char_value = c.to_digit(36).unwrap()-9;
                    }
                    matched_badges.push(char_value);
                    break;
                }
            }
        }
    }
    let sum = matched_chars.iter().sum::<u32>();
    println!("Sum: {}", sum);
    let sum_badge = matched_badges.iter().sum::<u32>();
    println!("Sum badge: {}", sum_badge);
}
