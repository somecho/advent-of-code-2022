use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

fn str_to_direction(dir: &str) -> Directions {
    match dir {
        "L" => Directions::Left,
        "R" => Directions::Right,
        "U" => Directions::Up,
        _ => Directions::Down,
    }
}

fn apply_direction(pos: Vec<i32>, dir: Directions) -> Vec<i32> {
    match dir {
        Directions::Up => vec![pos[0], pos[1] + 1],
        Directions::Down => vec![pos[0], pos[1] - 1],
        Directions::Left => vec![pos[0] - 1, pos[1]],
        Directions::Right => vec![pos[0] + 1, pos[1]],
    }
}

fn touching(pos1: Vec<i32>, pos2: Vec<i32>) -> bool {
    let x_dist = (pos1[0] - pos2[0]).abs();
    let y_dist = (pos1[1] - pos2[1]).abs();
    x_dist < 2 && y_dist < 2
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let instructions = input.split("\n").into_iter()
        .map(|i| {
            let tokens = i.trim().split_whitespace();
            let direction = str_to_direction(tokens.clone().nth(0).unwrap());
            let num_steps = tokens.clone().nth(1).unwrap().parse::<usize>().unwrap();
            vec![direction; num_steps]
        }).flatten().collect::<Vec<Directions>>();
    //PART ONE
    let mut head = vec![0, 0];
    let mut tail = vec![0, 0];
    let mut history: Vec<Vec<i32>> = Vec::new();
    for i in instructions.clone() {
        let new_head = apply_direction(head.clone(), i);
        if !touching(new_head.clone(), tail.clone()) {
            tail = head.clone();
        }
        head = new_head.clone();
        history.push(tail.clone());
    }
    let history = history.into_iter().collect::<HashSet<Vec<i32>>>();
    println!("part one: {}", history.len());
    //PART TWO
    let mut knots = vec![vec![0, 0]; 10];
    let mut history: Vec<Vec<i32>> = Vec::new();
    for i in instructions.clone() {
        knots[0] = apply_direction(knots[0].clone(), i);
        for j in 1..10 {
            if !touching(knots[j - 1].clone(), knots[j].clone()) {
                //check same column
                if knots[j - 1].clone()[0] == knots[j].clone()[0] {
                    if knots[j - 1].clone()[1] > knots[j].clone()[1] {
                        knots[j][1] += 1;
                    } else {
                        knots[j][1] -= 1;
                    }
                } else if knots[j - 1].clone()[1] == knots[j].clone()[1] {
                    if knots[j - 1].clone()[0] > knots[j].clone()[0] {
                        knots[j][0] += 1;
                    } else {
                        knots[j][0] -= 1;
                    }
                } else {
                    //neither same row nor column
                    if knots[j - 1].clone()[1] > knots[j].clone()[1] {
                        knots[j][1] += 1;
                    } else {
                        knots[j][1] -= 1;
                    }
                    if knots[j - 1].clone()[0] > knots[j].clone()[0] {
                        knots[j][0] += 1;
                    } else {
                        knots[j][0] -= 1;
                    }
                }
            }
        }
        history.push(knots[9].clone());
    }
    let history = history.into_iter().collect::<HashSet<Vec<i32>>>();
    println!("part two: {}", history.len());
}
