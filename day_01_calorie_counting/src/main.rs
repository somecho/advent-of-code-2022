use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let split = contents.split("\n");
    let mut bins: Vec<Vec<&str>> = vec![Vec::new()];
    let mut run  = 0;
    for s in split {
        match s {
            s if s == "\r" || s == "" => {
                run += 1;
                bins.push(Vec::new());
            },
            _ => {
                bins[run].push(s);
            }
        }
    }
    let mut bins = bins.into_iter().map(|bin|{
        bin.into_iter().map(|calories|{
            match calories.trim().parse::<i32>() {
                Ok(v) => v,
                Err(_) => 0
            }
        }).sum()
    }).collect::<Vec<i32>>();
    bins.sort();
    bins.reverse();
    //Part one
    let most = bins[0];
    println!("{:?}",most);
    //Part two
    let most:i32 = bins[..3].to_vec().iter().sum();
    println!("{:?}",most);
}
