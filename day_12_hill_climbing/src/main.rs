use std::collections::VecDeque;
use std::collections::HashMap;

fn shortest_step_to_point(start: (i32,i32),end:(i32,i32),heightmap:Vec<Vec<i32>>)->i32{
    let width = heightmap[0].len() as i32;
    let height = heightmap.len() as i32;
    let directions = [(0, 1), (0, -1), (-1, 0), (1, 0)];
    let mut q = VecDeque::from([start.clone()]);
    let mut graph = HashMap::new();
    let mut count = 0;
    let mut pos_history = vec![start.clone()];
    let mut found = false;
    loop {
        for position in q.clone() {
            for direction in directions.clone() {
                let x = position.0 + direction.0;
                let y = position.1 + direction.1;
                if x < 0 || x >= width || y < 0 || y >= height {
                    continue;
                }
                let my_height = heightmap[position.1 as usize][position.0 as usize];
                let other_height = heightmap[y as usize][x as usize];
                if other_height - my_height < 2
                    && !pos_history.contains(&(x, y))
                {
                    //if neighbor is good and hasn't been visited, add to graph
                    graph.entry(position).or_insert(Vec::new());
                    graph.entry(position).and_modify(|v| {
                        v.push((x, y));
                    });
                    //add to queue
                    q.push_back((x, y));
                    pos_history.push((x, y));

                    if (x, y) == end {
                        found = true;
                        break;
                    }
                }
            }
            //after checking all neighbors, pop queue
            q.pop_front();
        }
        count += 1;
        if count > 10000 || found {
            break;
        }
    }
    count
}
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();
    let mut start: (i32, i32) = (0, 0);
    let mut starting_points: Vec<(i32,i32)> = Vec::new();
    let mut end: (i32, i32) = (0, 0);
    let heightmap = input.clone().lines().enumerate()
        .map(|(y, line)| {
            line.chars().enumerate()
                .map(|(x, c)| {
                    match c {
                        'a' => {
                            starting_points.push((x as i32,y as i32));
                            c as i32
                        }
                        'b'..='z' => c as i32,
                        'S' => {
                            start = (x as i32, y as i32);
                            'a' as i32
                        }
                        'E' => {
                            end = (x as i32, y as i32);
                            'z' as i32
                        }
                        _ => {
                            0
                        }
                    }
                }).collect::<Vec<i32>>()
        }).collect::<Vec<Vec<i32>>>();
    let part_one = shortest_step_to_point(start,end,heightmap.clone());
    println!("Part one: {}",part_one);

    let least_steps_taken = starting_points.into_iter().map(|p|{
        shortest_step_to_point(p,end,heightmap.clone())
    }).min().unwrap();
    println!("Part two: {}",least_steps_taken);

}
