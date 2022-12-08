use std::{fs::read_to_string, collections::HashMap};


struct Directory {
    files: Vec<u32>,
    subdirs: HashMap<String, Directory>,
    size: u32,
}


impl Directory {
    fn populate_sizes(&mut self) {
        for dir in self.subdirs.values_mut() {
            dir.populate_sizes();
        }
        self.size = self.files.iter().sum::<u32>() + self.subdirs.values().map(|subdir| subdir.size).sum::<u32>();
    }
}


fn move_active_dir<'a>(dir: &'a mut Directory, path: &'_ [&str]) -> &'a mut Directory {
    let mut active_dir = dir;
    for part in path.iter().skip(1) {
        active_dir = active_dir.subdirs.get_mut(*part).unwrap();
    }
    active_dir
}

fn calc_part1(root: &Directory) -> u32 {
    let subdir_size = root.subdirs.values().map(calc_part1).sum::<u32>();
    if root.size < 100000 { return subdir_size+root.size; }
    subdir_size
}


fn main() {
    let filename = "input.txt";
    let data: &str = &read_to_string(filename).expect("File not found");
    let mut path = vec![];
    let mut root = Directory {
        files: vec![],
        subdirs: HashMap::new(),
        size: 0,
    };
    let mut active_dir = &mut root;
    for line in data.lines() {
        if let Some(dir) = line.strip_prefix("$ cd ") {
            if dir == ".." {
                path.pop();
            } else {
                path.push(dir);
            }
            active_dir = move_active_dir(&mut root, &path);
            continue;
        }
        if line.starts_with("$ ls") {
            continue;
        }
        if line.starts_with("dir") {
            let dir_name = &line[4..];
            active_dir.subdirs.insert(dir_name.to_string(), Directory {
                files: vec![],
                subdirs: HashMap::new(),
                size: 0,
            });
            continue;
        }
        if line.starts_with(|c: char| c.is_numeric()) {
            let size = line.split_whitespace().next().unwrap().parse::<u32>().unwrap();
            active_dir.files.push(size);
            continue;
        }
    }
    root.populate_sizes();
    println!("Part 1: {}", calc_part1(&root));
    let space_needed = 30_000_000 - (70_000_000 - root.size);
    let mut dirs = Vec::from_iter(root.subdirs.values());
    let mut min = 70_000_000;
    while let Some(dir) = dirs.pop() {
        if dir.size < min && dir.size > space_needed {
            min = dir.size;
        }
        dirs.extend(dir.subdirs.values());
    }
    println!("Part 2: {}", min);
}
