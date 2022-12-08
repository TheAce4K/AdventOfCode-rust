use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
    let filename = "input.txt";
    let data: &str = &read_to_string(filename).expect("File not found");
    let line_len = data.lines().count();
    let mut visible: Vec<Vec<bool>> = vec![vec![false; line_len]; line_len];
    let mut row_high: char;
    let mut col_high: Vec<char> = vec!['0'; line_len]; 
    for (i, line) in data.lines().enumerate() {
        row_high = '0';
        for (j, c) in line.chars().enumerate() {
            if c > row_high {
                row_high = c;
                visible[i][j] = true;
            }
            if c > col_high[j] {
                col_high[j] = c;
                visible[i][j] = true;
            }
            if i == 0 || j == 0 || i == line_len-1 || j == line_len-1 { visible[i][j] = true; }
        }
    }
    col_high = vec!['0'; line_len]; 
    for (i, line) in data.lines().rev().enumerate() {
        row_high = '0';
        for (j, c) in line.chars().rev().enumerate() {
            if c > row_high {
                row_high = c;
                visible[line_len-1-i][line_len-1-j] = true;
            }
            if c > col_high[line_len-1-j] {
                col_high[line_len-1-j] = c;
                visible[line_len-1-i][line_len-1-j] = true;
            }
        }
    }

    let trees_visible: u32 = visible.iter().map(|col| col.iter().map(|x| {
        match x {
            true => 1,
            false => 0,
        }
    }).sum::<u32>()).sum();
    println!("trees visible: {}", trees_visible);
    let mut map = vec![vec!['0'; line_len]; line_len];
    for (i, line) in data.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            map[j][i] = c;
        }
    }
    let mut ratings = vec![vec![0; line_len]; line_len];
    for (i, line) in data.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let left = vec![line[..j].chars().rev().position(|x| x >= c), Some(line[..j].len()) ];
            let right = vec![line[j+1..].chars().position(|x| x >= c), Some(line[j+1..].len()) ];
            let up = vec![map[j][..i].iter().rev().position(|x| x >= &c), Some(map[j][..i].len()) ];
            let down = vec![map[j][i+1..].iter().position(|x| x >= &c), Some(map[j][i+1..].len()) ];
            let rating = vec![left, right, up, down].iter().map(|x| {
                if x[0].is_some() { 
                    x[0].unwrap()+1
                }
                else { 
                    x[1].unwrap()
                }
            }).product();
            ratings[i][j] = rating;
        }
    }
    let max_rating = ratings.iter().map(|i| i.iter().max().unwrap()).max().unwrap();
    println!("max rating: {}", max_rating)
}
