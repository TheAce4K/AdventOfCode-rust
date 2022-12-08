use std::{fs::read_to_string, collections::{VecDeque, HashMap}};


fn part1(data: &str) {
    let mut seq: HashMap<char, u8> = HashMap::with_capacity(26);
    let mut wind: VecDeque<char> = VecDeque::with_capacity(4);
    let mut idx: usize = 0;
    for c in data.chars() {
        idx += 1;
        let new_val = seq.entry(c).or_insert(0);
        *new_val += 1;
        if idx > 4 {
            let pop_val = seq.entry(wind.pop_front().expect("empty")).or_insert(1);
            *pop_val -= 1;
        }
        wind.push_back(c);
        if idx > 4 && wind.iter().map(|c| seq.get(c).expect("not found")).all(|&x| x == 1) {
            println!("part 1: {} at index: {}", wind.iter().collect::<String>(), idx);
            break;
        }
    }
}

// same as part 1 but optimized
// time complexity: O(n)
// space complexity: O(1) given that we know the alphabet size
// hashmap unneccessary since we know the alphabet size
// Vec would work just as well, probably faster
fn part2(data: &str) {
    let mut seq: HashMap<char, u8> = HashMap::with_capacity(26);
    let mut idx: usize = 0;
    let mut douplicate_counter: usize = 0;
    for c in data.chars() {
        idx += 1;
        let new_val = seq.entry(c).or_insert(0);
        if *new_val >= 1 { douplicate_counter += 1; }
        *new_val += 1;
        if idx > 14 {
            let c_pop = data.chars().nth(idx-14-1).expect("empty");
            let pop_val = seq.entry(c_pop).or_insert(1);
            if *pop_val > 1 { douplicate_counter -= 1; }
            *pop_val -= 1;
        }
        if idx > 14 && douplicate_counter == 0 {
            println!("part 2 at index: {}", idx);
            break;
        }
    }
}


fn main() {
    let filename = "input.txt";
    let input = read_to_string(filename).expect("could not read file");
    part1(&input);
    part2(&input);

}
