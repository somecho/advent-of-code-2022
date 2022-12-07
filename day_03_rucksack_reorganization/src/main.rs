use std::fs;

fn priority(item: char) -> usize {
    match item as usize { //map priorities
        i if i >= 'a' as usize && i <= 'z' as usize => i - 96,
        i if i >= 'A' as usize && i <= 'Z' as usize => i - 38,
        i => i
    }
}

fn common_substring(str1: &str, str2: &str) -> String {
    str1.chars()
        .into_iter()
        .map(|c| {
            match c {
                c if str2.contains(c) => c,
                _ => '.'
            }
        })
        .filter(|c| *c != '.')
        .collect::<String>()
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.split("\n");
    //PART ONE
    let total_priority: usize = lines.clone().into_iter().map(|l| {
        let rucksack_size = l.trim().len();
        let compartment1 = l.get(0..rucksack_size / 2).unwrap();
        let compartment2 = l.get(rucksack_size / 2..).unwrap();
        let res = common_substring(compartment1, compartment2);
        priority(res.chars().nth(0).unwrap())
    }).sum();
    println!("{}", total_priority);
    //PART TWO
    let rucksacks = lines.collect::<Vec<&str>>();
    let num_rucksacks = rucksacks.len();
    let groups = (0..num_rucksacks / 3).map(|i| {
        vec![rucksacks[i * 3].trim(), rucksacks[i * 3 + 1].trim(), rucksacks[i * 3 + 2].trim()]
    }).collect::<Vec<Vec<&str>>>();
    let total_priority: usize = groups.iter().map(|group| {
        //find common badge between line 1 & 2, then 2 and 3
        let res = common_substring(group[0], group[1]);
        let res2 = common_substring(res.as_str(), group[2]);
        priority(res2.chars().nth(0).unwrap())
    }).sum();
    println!("{}", total_priority);
}
