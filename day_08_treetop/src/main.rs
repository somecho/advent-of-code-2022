fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let trees = contents.split("\n").into_iter()
        .map(|l| {
            l.trim().chars().into_iter().map(|tree| {
                tree.to_digit(10).unwrap()
            }).collect::<Vec<u32>>()
        }).collect::<Vec<Vec<u32>>>();
    let grid_height = trees.len();
    let grid_width = trees[0].len();
    let visibility = trees.clone().into_iter().enumerate()
        .map(|(y, row)| {
            row.into_iter().enumerate()
                .map(|(x, tree)| {
                    if y == 0 { return 1; }
                    if y == grid_height - 1 { return 1; }
                    if x == 0 { return 1; }
                    if x == grid_width - 1 { return 1; }
                    let mut top = true;
                    let mut bottom = true;
                    let mut right = true;
                    let mut left = true;
                    for i in 0..y {
                        let other = trees.clone()[i][x];
                        if other >= tree { top = false }
                    }
                    for i in y + 1..grid_height {
                        let other = trees.clone()[i][x];
                        if other >= tree { bottom = false }
                    }
                    for i in 0..x {
                        let other = trees.clone()[y][i];
                        if other >= tree { left = false }
                    }
                    for i in x + 1..grid_width {
                        let other = trees.clone()[y][i];
                        if other >= tree { right = false }
                    }
                    let visible = top || bottom || right || left;
                    match visible {
                        true => 1,
                        false => 0
                    }
                }).collect::<Vec<u32>>()
        }).collect::<Vec<Vec<u32>>>();
    let total_visible: u32 = visibility.into_iter().map(|row| {
        row.into_iter().sum::<u32>()
    }).sum();
    println!("{}", total_visible);
    //part two
    let scenic_score = trees.clone().into_iter().enumerate()
        .map(|(y, row)| {
            row.into_iter().enumerate()
                .map(|(x, tree)| {
                    let mut scores = vec![0,0,0,0];
                    for i in (0..y).rev() {
                        let other = trees.clone()[i][x];
                        if other < tree { scores[0]+=1} else { scores[0]+=1; break;}
                    }
                    for i in y+1..grid_height {
                        let other = trees.clone()[i][x];
                        if other < tree { scores[1]+=1} else { scores[1]+=1;break;}
                    }
                    for i in (0..x).rev() {
                        let other = trees.clone()[y][i];
                        if other < tree { scores[2]+=1} else { scores[2]+=1;break;}
                    }
                    for i in x+1..grid_width {
                        let other = trees.clone()[y][i];
                        if other < tree { scores[3]+=1} else { scores[3]+=1;break;}
                    }
                    scores[0] * scores[1] * scores[2] * scores[3]
                }).collect::<Vec<u32>>()
        }).collect::<Vec<Vec<u32>>>();
    let max_score = scenic_score.into_iter()
        .map(|row|{
            row.into_iter().max().unwrap()
        }).max().unwrap();
    println!("part two: {}",max_score)

}
