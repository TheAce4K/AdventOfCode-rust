use std::{fs::read_to_string, time::Instant};


fn part1(data: &str) -> u32 {
    let line_len = data.lines().count();
    // part 1 
    // time compexity according to my calculations if n is number of trees then we
    // loop through each element 2 times for part 1 so O(2n) = O(n)
    // if we say that n is the size of one side of the grid then solution is O(n^2)
    // memmory is same as time complexity since i save the result in a matrix. 
    // Memory complexity could be improved
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
    trees_visible
}


fn part2(data: &str) -> usize {
    // let n be number of trees. then time complexity is O(nlog(n)) if we assume that search is
    // log(n). I feel like there is a better algorithm for this? Memory complexity O(n^2) since im
    // storing the input in matrix. if we could find a solution without this we would have O(1) 
    // memory complexity but dont know how to loop vertically over string with lines.
    let line_len = data.lines().count();
    let mut map = vec![vec!['0'; line_len]; line_len];
    for (i, line) in data.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            map[j][i] = c;
        }
    }
    let mut running_rating_max: usize = 0;
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
            if rating > running_rating_max { running_rating_max = rating; }
        }
    }
    running_rating_max
}

fn main() {
    println!("Hello, world!");
    let filename = "input.txt";
    let data: &str = &read_to_string(filename).expect("File not found");
    let t0 = Instant::now();
    let trees_visible = part1(data);
    let t1 = Instant::now();
    println!("trees visible: {}", trees_visible);
    let t2 = Instant::now();
    let running_rating_max = part2(data);  
    let t3 = Instant::now();
    println!("running max rating: {}", running_rating_max);
    println!("First part took: {:?}, Second part took: {:?}", t1.duration_since(t0), t3.duration_since(t2))
}
