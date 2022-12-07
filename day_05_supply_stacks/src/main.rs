fn move_crate(stacks:Vec<Vec<char>>,from:i32,to:i32)->Vec<Vec<char>>{
    stacks.clone().into_iter().enumerate().map(|(i,stack)|{
        match i as i32 {
            i if i == from =>  {
                stacks[i as usize][1..].to_vec()
            },
            i if i == to => {
                [&stacks[from as usize][0..1],&stack[..]].concat().to_vec()
            }
        _ => stack
        }
    }).collect::<Vec<Vec<char>>>()
}
fn move_crate_compound(stacks:Vec<Vec<char>>,count: i32,from:i32,to:i32)->Vec<Vec<char>>{
    stacks.clone().into_iter().enumerate().map(|(i,stack)|{
        match i as i32 {
            i if i == from => {
                stacks[i as usize][count as usize..].to_vec()
            },
            i if i == to => {
                [&stacks[from as usize][0..count as usize],&stack[..]].concat().to_vec()
            },
            _=>stack
        }
    }).collect::<Vec<Vec<char>>>()
}
fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let parts = contents.split("\r\n\r\n").collect::<Vec<&str>>();
    //Split input into crates and instructions section
    let crates = parts[0];
    let instructions = parts[1]
        .split("\n")
        .into_iter()
        .map(|l| {
            let tokens = l.trim().split_whitespace().collect::<Vec<&str>>();
            vec![tokens[1], tokens[3], tokens[5]].into_iter().map(|num| {
                num.parse::<i32>().unwrap()
            }).collect::<Vec<i32>>()
        }).collect::<Vec<Vec<i32>>>();
    //process crates
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];
    for c in crates.split("\n") {
        for i in 0..=9 {
            match c.chars().nth(i * 4 + 1) {
                Some(T) => {
                    if T != ' ' && T.to_string().parse::<i32>().is_err() {
                        stacks[i].push(T)
                    }
                }
                None => ()
            }
        }
    }
    //PART ONE
    let mut result = stacks.clone();
    for i in instructions.clone() {
        for j in 0..i[0]{
            result = move_crate(result,i[1]-1,i[2]-1);
        }
    }
    let result = result
        .iter()
        .map(|s|s.first().unwrap()).collect::<Vec<&char>>();
    println!("{:?}",result);
    //PART TWO
    let mut result = stacks.clone();
    for i in instructions.clone() {
        result = move_crate_compound(result,i[0],i[1]-1,i[2]-1);
    }
    let result = result
        .iter()
        .map(|s|s.first().unwrap()).collect::<Vec<&char>>();
    println!("{:?}",result);
}
