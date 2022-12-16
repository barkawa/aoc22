use regex::Regex;

#[derive(PartialEq, Eq)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Sensor {
    location: Coord,
    closest_beacon: Coord,
}

pub fn day15() {
    let input = super::get_input();

    let regex =
        Regex::new(r"Sensor at x=(\S+), y=(\S+): closest beacon is at x=(\S+), y=(\S+)").unwrap();

    let sensors: Vec<Sensor> = input
        .iter()
        .map(|line| regex.captures(line).unwrap())
        .map(|captures| Sensor {
            location: Coord {
                x: captures[1].parse().unwrap(),
                y: captures[2].parse().unwrap(),
            },
            closest_beacon: Coord {
                x: captures[3].parse().unwrap(),
                y: captures[4].parse().unwrap(),
            },
        })
        .collect();

    let mut test_pos = Coord { x: 0, y: 2_000_000 };
    let mut count: u64 = 0;

    for x in -10_000_000..10_000_000 {
        test_pos.x = x;

        let can_be_beacon;

        if sensors
            .iter()
            .map(|s| &s.closest_beacon)
            .any(|beacon_pos| beacon_pos == &test_pos)
        {
            can_be_beacon = true;
        } else if sensors
            .iter()
            .map(|sensor| sensor.location.distance(&test_pos) 
                          <= sensor.location.distance(&sensor.closest_beacon))
            .any(|b| b)
        {
            can_be_beacon = false;
        } else {
            can_be_beacon = true;
        }

        if !can_be_beacon {
            count += 1;
        }
    }

    dbg!(count);
}
