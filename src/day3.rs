use std::collections::HashSet;

pub fn day3() {
    let input = super::get_input();

    let total_score: u32 = input
        .iter()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(a, b)| {
            let a_set: HashSet<char> = a.chars().collect();
            let b_set: HashSet<char> = b.chars().collect();
            a_set.intersection(&b_set).next().unwrap().clone()
        })
        .map(priority)
        .sum();

    dbg!(total_score);

    let group_score: u32 = input.chunks(3).map(triple_intersection).map(priority).sum();

    dbg!(group_score);
}

fn triple_intersection(chunk: &[String]) -> char {
    assert!(chunk.len() == 3);

    let mut sets: [HashSet<char>; 3] = Default::default();

    for i in 0..3 {
        sets[i] = HashSet::from_iter(chunk[i].chars());
    }

    sets[0]
        .intersection(&sets[1])
        .copied()
        .collect::<HashSet<char>>()
        .intersection(&sets[2])
        .next()
        .unwrap()
        .clone()
}

fn priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 + 1 - 'a' as u32
    } else if c.is_ascii_uppercase() {
        c as u32 + 27 - 'A' as u32
    } else {
        unreachable!();
    }
}
