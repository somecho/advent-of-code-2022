fn range_contains_range(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
    start2 <= start1 && end2 >= end1
}

fn ranges_overlap(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
    let range1 = (start1..=end1).collect::<Vec<i32>>();
    let range2 = (start2..=end2).collect::<Vec<i32>>();
    for n in range1 {
        if range2.contains(&n) {
            return true;
        }
    }
    false
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let camps = lines.into_iter().map(|l| {
        let mut camp = Vec::new();
        let pair = l.trim().split(",");
        for p in pair {
            let range = p.split("-");
            for ends in range {
                let num = ends.parse::<i32>().unwrap();
                camp.push(num);
            }
        }
        camp
    }).collect::<Vec<Vec<i32>>>();
    //PART ONE
    let total_contains:i32 = camps.clone().into_iter().map(|a| {
        let b_in_a = range_contains_range(a[0], a[1], a[2], a[3]);
        let a_in_b = range_contains_range(a[2], a[3], a[0], a[1]);
        let contains = a_in_b || b_in_a;
        match contains {
            true => 1,
            false => 0
        }
    }).sum();
    println!("Part one: {}", total_contains);
    let total_overlaps: i32 = camps.clone().into_iter().map(|a|{
        match ranges_overlap(a[0],a[1],a[2],a[3]) {
            true => 1,
            false => 0
        }
    }).sum();
    println!("Part two: {}",total_overlaps);
}
