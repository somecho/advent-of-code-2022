use std::collections::HashMap;
fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let commands = content.split("\n");
    let mut paths: Vec<&str> = Vec::new();
    let mut dir_sizes = HashMap::<Vec<&str>, i32>::new();
    for c in commands {
        let tokens = c.trim().split_whitespace();
        let first_token = tokens.clone().nth(0).unwrap();
        match first_token {
            "$" => {
                let commands = tokens.clone().nth(1).unwrap();
                if commands == "cd" {
                    let path = tokens.clone().nth(2).unwrap();
                    match path {
                        ".." => { //cd-ing out, remove last el from paths
                            paths.remove(paths.len() - 1);
                        }
                        dir => { //cd-ing in, add to paths
                            paths.push(dir);
                            dir_sizes.entry(paths.clone()).or_insert(0);
                        }
                    }
                }
            }
            "dir" => {} //don't need to do anything
            _ => { //is a filesize, add to all dirs listed in path
                let file_size = tokens.clone().nth(0).unwrap().parse::<i32>().unwrap();
                for i in 0..paths.len() {
                    let dir_path = paths[0..i+1].to_vec();
                    dir_sizes.entry(dir_path)
                        .and_modify(|size| *size += file_size);
                }
            }
        }
    }
    let total_size: i32 = dir_sizes.clone().into_iter().map(|(_, s)| {
        match s {
            size if size <= 100_000 => size,
            _ => 0
        }
    }).sum();
    println!("part one: {:?}", total_size);
    //part two
    let used_space = dir_sizes.get(&vec!["/"]).unwrap();
    let to_delete =  *used_space - 40_000_000;
    let candidate = dir_sizes.clone().into_iter()
        .filter(|(dir,size)|*size>to_delete)
        .map(|(dir,size)|size)
        .min()
        .unwrap();
    println!("part two: {}",candidate);
}
