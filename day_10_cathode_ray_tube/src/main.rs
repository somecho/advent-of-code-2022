fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();
    let mut x = 1;
    let mut cycles: Vec<i32> = Vec::new();
    for l in lines {
        match l.split_once(" ") {
            Some(op) => {
                let value: i32 = op.1.parse().unwrap();
                cycles.push(x);
                cycles.push(x);
                x += value;
            }
            None => {
                cycles.push(x);
            }
        }
    }
    let signal_strength_sum = [20, 60, 100, 140, 180, 220].into_iter()
        .map(|i| {
            cycles.clone()[i - 1] * i as i32
        }).sum::<i32>();
    println!("Part One: {}", signal_strength_sum);

    println!("Part Two:");
    for y in 0..6 {
        for x in 0..40 {
            let dist = x as i32 - cycles.clone()[y*40+x];
            match dist.abs() {
                d if d < 2 => print!("#"),
                _ => print!(".")
            }
        }
        print!("\n");
    }
}
