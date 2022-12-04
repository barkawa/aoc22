pub fn day4() {
    let input = super::get_input();

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;

    for line in &input {
        let n: Vec<u32> = line.split([',', '-']).map(|s| s.parse().unwrap()).collect();
        part1 += ((n[0] <= n[2]) && (n[1] >= n[3]) || (n[0] >= n[2]) && (n[1] <= n[3])) as u32;
        part2 += !((n[1] < n[2]) || (n[0] > n[3])) as u32;
    }

    println!("part1: {part1}, part2: {part2}");
}