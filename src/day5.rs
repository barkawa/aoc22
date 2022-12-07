enum Part {
    One,
    Two,
}

pub fn day5() {
    let input = super::get_input();

    let stacks = parse_crate_stacks(&input);

    let resulting_stacks = make_moves(&input, &stacks, Part::One);
    let topmost_crates: String = resulting_stacks.iter().map(|s| s.last().unwrap()).collect();
    println!("part1: {topmost_crates}");

    let resulting_stacks = make_moves(&input, &stacks, Part::Two);
    let topmost_crates: String = resulting_stacks.iter().map(|s| s.last().unwrap()).collect();
    println!("part1: {topmost_crates}");
}

fn make_moves(input: &[String], stacks: &Vec<Vec<char>>, part: Part) -> Vec<Vec<char>> {
    let mut stacks = stacks.clone();

    let first_move_idx = input
        .iter()
        .position(|line| line.starts_with("move"))
        .unwrap();

    let moves = &input[first_move_idx..];

    for m in moves {
        let (n_crates_moved, from, to) = parse_move(&m);

        match part {
            Part::One => {
                for _ in 0..n_crates_moved {
                    if let Some(crate_id) = stacks[from].pop() {
                        stacks[to].push(crate_id);
                    }
                }
            }
            Part::Two => {
                let len = stacks[from].len();
                let moved_crates: Vec<_> = stacks[from].drain(len - n_crates_moved..).collect();
                stacks[to].extend(moved_crates);
            }
        }
    }

    stacks
}

fn parse_move(line: &str) -> (usize, usize, usize) {
    let a: Vec<usize> = line
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    (a[0], a[1]-1, a[2]-1)
}

fn parse_crate_stacks(input: &[String]) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Default::default();

    // find the line that indexes the crate columns
    // (looks like this: " 1   2   3   4 ")
    let crate_col_indices_idx = input.iter().position(|line| !line.contains("[")).unwrap();
    let crate_col_indices = &input[crate_col_indices_idx];

    // now go upwards from the position of the crate column index and read the crate ID's
    for (x, _) in crate_col_indices
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_ascii_digit())
    {
        stacks.push(vec![]);
        for y in (0..crate_col_indices_idx).rev() {
            let crate_id: char = input[y].chars().nth(x).unwrap();
            if crate_id != ' ' {
                stacks.last_mut().unwrap().push(crate_id);
            }
        }
    }

    stacks
}
