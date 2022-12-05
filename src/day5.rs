pub fn day5() {
    let input = super::get_input();

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

    dbg!(stacks);

}
