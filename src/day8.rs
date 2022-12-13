use std::{fmt::Display, vec};

pub fn day8() {
    let input = super::get_input();

    let mut forest: Vec<Vec<i8>> = Vec::new();

    for line in input.iter() {
        let nums = line.chars().map(|c| c.to_string().parse::<i8>().unwrap());
        forest.push(Vec::new());
        forest.last_mut().unwrap().extend(nums);
    }

    // part1
    let mut viz = vec![vec![false; forest.len()]; forest.len()];
    check_visibility(&forest, &mut viz);

    let sum: i64 = viz.iter().flatten().map(|&b| b as i64).sum();
    dbg!(sum);

    // part2
    let score = get_best_scenic_score(&forest);
    dbg!(score);
}


fn get_best_scenic_score(forest: &[Vec<i8>]) -> i64 {
    let mut best = 0;
    
    for i in 0..forest.len() {
        for j in 0..forest.len() {                        
            let score = get_scenic_score(forest, (i, j));

            if score > best {
                best = score;
            }
        }
    }

    best
}

fn get_scenic_score(forest: &[Vec<i8>], idx: (usize, usize)) -> i64 {
    let (i, j) = idx;
    let h = forest[i][j];
    let mut score = 1;

    let mut d = 0;
    for ii in (i+1)..forest.len() {
        d += 1;
        if forest[ii][j] >= h {
            break;
        }
    }
    score *= d;

    let mut d = 0;
    for ii in (0..i).rev() {
        d += 1;
        if forest[ii][j] >= h {
            break;
        }
    }
    score *= d;

    let mut d = 0;
    for jj in (j+1)..forest.len() {
        d += 1;
        if forest[i][jj] >= h {
            break;
        }
    }
    score *= d;

    let mut d = 0;
    for jj in (0..j).rev() {
        d += 1;
        if forest[i][jj] >= h {
            break;
        }
    }
    score *= d;


    score as i64
}

fn check_visibility(
    forest: &[Vec<i8>],
    viz: &mut Vec<Vec<bool>>,
) {
    let l = forest.len();

    for i in 0..l {
        let mut tallest_down = -1;
        let mut tallest_up = -1;
        let mut tallest_left = -1;
        let mut tallest_right = -1;

        for j in 0..l {
            // down
            if forest[j][i] > tallest_down {
                viz[j][i] = true;
                tallest_down = forest[j][i];
            }

            // up
            if forest[l - (j+1)][i] > tallest_up {
                viz[l - (j+1)][i] = true;
                tallest_up = forest[l - (j+1)][i];
            }

            // left
            if forest[i][l - (j+1)] > tallest_left {
                viz[i][l - (j+1)] = true;
                tallest_left = forest[i][l - (j+1)];
            }
            
            // right
            if forest[i][j] > tallest_right {
                viz[i][j] = true;
                tallest_right = forest[i][j];
            }


        }
    }
}

fn print<T>(forest: &[Vec<T>])
    where T: Display {
    for row in forest {
        for tree in row {
            print!("{tree}");
        }
        println!();
    }
    println!();
}