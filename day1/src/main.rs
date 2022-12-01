use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

// Read data from file
fn read_data(filename: &str) -> Vec<i32> {
    let mut data = Vec::new();
    let file = File::open(filename)
        .expect("File not found");
    let buf_reader = BufReader::new(file);
    let mut count: i32 = 0;
    for line in buf_reader.lines() {
        if line.as_ref().unwrap() == "" {
            data.push(count);
            count = 0;
        } else {
            let num: i32 = line.unwrap().parse()
                .expect("Not a number");
            count += num;
        }
    }
    return data;
}

fn top_3(data: &Vec<i32>) -> Vec<i32> {
    let mut top = vec![3, 2, 1];
    for val in data.iter() {
        match val {
            val if val > &top[0] => { top.pop(); top.insert(0, *val); },
            val if val < &top[0] && val > &top[1] => {top.pop(); top.insert(1, *val);},
            val if val < &top[1] && val > &top[2] => {top.pop(); top.insert(2, *val);},
            val if val < &top[2] => {},
            _ => { panic!("Not valid value"); }
        }
    }
    return top;
}


fn main() {
    let data: Vec<i32> = read_data("data.txt");
    for val in &data {
        println!("{}", val);
    }
    println!("Max calories carried by one elf: {}", data.iter().max().unwrap());
    let top = top_3(&data);
    println!("Top 3: {}, {}, {}", top[0], top[1], top[2]);
    println!("Sum of top 3: {}", top.iter().sum::<i32>());
}
