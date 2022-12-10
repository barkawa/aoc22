use std::collections::HashMap;

pub fn day6 () {
    let input = super::get_input();

    assert!(input.len() == 1);
    assert!(input[0].is_ascii());

    let input = input[0].as_bytes();

    for (idx, window) in input.windows(4).enumerate() {
        if is_start_of_packet(window) {
            let start_of_packet = idx + 4;
            println!("Start of packet: {}", start_of_packet);
            break;
        }
    }

    // part2
    for (idx, window) in input.windows(14).enumerate() {
        let mut map: HashMap<u8, u32> = HashMap::new();
        for c in window {
            map.entry(*c).and_modify(|e| *e += 1).or_insert(1);
        }

        if map.values().all(|&amount| amount < 2) {
            let start_of_packet = idx + 14;
            println!("Start of packet: {}", start_of_packet);
            break;
        }
    }
}

fn is_start_of_packet(a: &[u8]) -> bool {
    a[0] != a[1] && a[2] != a[3] && a[0] != a[2] && a[1] != a[3] && a[0] != a[3] && a[1] != a[2]
}
