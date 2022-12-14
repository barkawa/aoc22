use std::{fmt, mem::swap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GridCell {
    Air,
    Sand,
    Rock,
}

use GridCell::*;

impl fmt::Display for GridCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Air => " ",
            Rock => "O",
            Sand => "*",
        };

        write!(f, "{}", c)
    }
}

fn print(grid: &[Vec<GridCell>]) {
    for line in grid {
        for c in line[400..].iter() {
            print!("{c}");
        }
        println!();
    }
}

fn parse_input(input: &[String]) -> Vec<Vec<GridCell>>{
    let mut grid = vec![vec![Air; 600]; 200];

    for line in input {
        let coords: Vec<(usize, usize)> = line
            .split(" -> ")
            .map(|coord| coord.split_once(",").unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect();

        for pair in coords.windows(2) {
            let (mut x1, mut y1) = pair[0];
            let (mut x2, mut y2) = pair[1];

            if x1 == x2 {
                if y1 > y2 {
                    swap(&mut y1, &mut y2);
                }
                for y in y1..=y2 {
                    grid[y][x1] = Rock;
                }
            } else if y1 == y2 {
                if x1 > x2 {
                    swap(&mut x1, &mut x2);
                }
                for x in x1..=x2 {
                    grid[y1][x] = Rock;
                }
            } else {
                unreachable!();
            }
        }        
    }

    grid
}

struct Coord {
    x: usize,
    y: usize,
}

pub fn day14() {
    let input = super::get_input();

    let mut grid = parse_input(&input);

    'sand: loop { // TODO
        let mut s = Coord {x: 500, y: 0};

        'time: loop {
            if s.y + 1 >= grid.len() {
                break 'sand;
            }
            if grid[s.y + 1][s.x] == Air {  // below
                s.y += 1;
            } else if grid[s.y+1][s.x - 1] == Air { // bottom left
                s.y += 1;
                s.x -= 1;
            } else if grid[s.y+1][s.x + 1] == Air { // bottom right
                s.y += 1;
                s.x += 1;
            } else { // nothing free, deposit sand
                grid[s.y][s.x] = Sand;
                break 'time;
            }
        }
    }

    let count = grid.iter().flatten().filter(|&&x| x == Sand).count();

    print(&grid);

    dbg!(count);

}