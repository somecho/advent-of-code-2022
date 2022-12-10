fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let mut calories = contents.split("\r\n\r\n").map(|bag|{
        bag.lines().map(|calorie|{
            calorie.trim().parse::<i32>().unwrap()
        }).sum()
    }).collect::<Vec<i32>>();
    calories.sort();
    calories.reverse();
    println!("Part one: {}",calories[0]);
    println!("Part two: {}", calories[..3].iter().sum::<i32>());
}
