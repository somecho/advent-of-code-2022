use std::fs;
// 0 - rock, 1 - paper, 2 - scissors
fn move_index(m: &str)->usize{
    match m {
        "A" | "X" => 0,
        "B" | "Y" => 1,
        "C" | "Z" => 2,
        _ => 0
    }
}
fn main() {
    let strategy = fs::read_to_string("input.txt").unwrap();
    let lines = strategy.split("\n");
    //PART ONE
    let outcome_table = vec![
        vec![4,1,7], //rock
        vec![8,5,2], //paper
        vec![3,9,6]]; //scissors
    let total_points: i32 = lines.clone().into_iter().map(|round|{
        let opponent = move_index(round.get(0..1).unwrap());
        let response = move_index(round.get(2..3).unwrap());
        outcome_table[response][opponent]
    }).sum();
    println!("part one: {}",total_points);
    //PART TWO
    let match_table = vec![
        vec![2,0,1], //rock
        vec![0,1,2], //paper
        vec![1,2,0] //scissors
    ];
    let total_points: i32 = lines.clone().into_iter().map(|round|{
        let opponent = move_index(round.get(0..1).unwrap());
        let response = move_index(round.get(2..3).unwrap());
        let match_up = match_table[opponent][response];
        outcome_table[match_up][opponent];
    }).sum();
    println!("part two: {}",total_points);
}
