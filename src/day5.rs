pub fn day5() {
    let input = super::get_input();

    let mut stacks = parse_crate_stacks(&input);

    make_moves(&input, &mut stacks);

    let topmost_crates: String = stacks.iter().map(|s| s.last().unwrap()).collect();
    println!("Topmost crates: {topmost_crates}");
}

fn make_moves(input: &[String], stacks: &mut Vec<Vec<char>>) {
    let first_move_idx = input
        .iter()
        .position(|line| line.starts_with("move"))
        .unwrap();

    let moves = &input[first_move_idx..];

    for m in moves {
        let a: Vec<usize> = m
            .split_ascii_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        assert!(a.len() == 3);

        let n_crates_moved = a[0];
        let from = a[1] - 1;
        let to = a[2] - 1;

        for _ in 0..n_crates_moved {
            if let Some(crate_id) = stacks[from].pop() {
                stacks[to].push(crate_id);
            }
        }
    }
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
