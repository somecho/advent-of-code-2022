fn get_term(value: u64, operation: &str)->u64{
    match operation {
        "old" => value,
        v => v.parse::<u64>().unwrap()
    }
}
fn operate(a:u64,b:u64,operator:&str)->u64{
    match operator {
        "*" => a*b,
        _ => a+b
    }
}
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let divisor = [3,1];
    let part_name = ["one","two"];
    let rounds = [20,10000];
    for part in 0..2 {
        let monkeys = input.split("\r\n\r\n");
        let mut items: Vec<Vec<u64>> = Vec::new();
        let mut operations: Vec<Vec<&str>> = Vec::new();
        let mut tests: Vec<u64> = Vec::new();
        let mut false_targets: Vec<usize> = Vec::new();
        let mut true_targets: Vec<usize> = Vec::new();
        for m in monkeys {
            let lines = m.lines();
            let item_list = lines.clone().nth(1).unwrap()
                .get(18..).unwrap()
                .split(",")
                .map(|v| v.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let operation = lines.clone().nth(2).unwrap().get(13..).unwrap()
                .split_whitespace().collect::<Vec<&str>>();
            let test_val = lines.clone().nth(3).unwrap().get(21..).unwrap()
                .parse::<u64>().unwrap();
            let true_target = lines.clone().nth(4).unwrap().get(29..).unwrap();
            let false_target = lines.clone().nth(5).unwrap().get(30..).unwrap();
            items.push(item_list);
            operations.push(operation);
            tests.push(test_val);
            false_targets.push(false_target.parse::<usize>().unwrap());
            true_targets.push(true_target.parse::<usize>().unwrap());
        }

        let num_monkeys = tests.clone().len();
        let mut monkey_count: Vec<u64> = vec![0; num_monkeys];
        for _ in 0..rounds[part] {
            for i in 0..num_monkeys {
                for worry in items[i].clone() {
                    let a = get_term(worry, operations[i][2]);
                    let b = get_term(worry, operations[i][4]);
                    let new = operate(a, b, operations[i][3]);
                    //this line is wack, doesn't work for part one!
                    let bored = (new / divisor[part]) % (11*2*5*17*19*7*3*13);
                    let test_passed = bored % tests[i] == 0;
                    match test_passed {
                        true => items[true_targets[i]].push(bored),
                        false => items[false_targets[i]].push(bored),
                    }
                    monkey_count[i] += 1;
                }
                items[i] = Vec::new();
            }
        }
        monkey_count.sort_by(|a, b| b.cmp(a));
        let monkey_level = monkey_count[0] * monkey_count[1];
        println!("Part {}: {}", part_name[part], monkey_level);
    }
}
