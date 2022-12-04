fn score(round: &str) -> u32 {
    /* 
        A / X = Rock
        B / Y = Paper
        C / Z = Scissors
    */

    match round {
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
        _ => unimplemented!(),
    }
}

fn score2(round: &str) -> u32{
    /*
        A = Rock
        B = Paper
        C = Scissors
        
        X = Loss
        Y = Draw
        Z = Win
    */

    match round {
        "A X" => 0 + 3,
        "A Y" => 3 + 1,
        "A Z" => 6 + 2,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 0 + 2,
        "C Y" => 3 + 3,
        "C Z" => 6 + 1,
        _ => unimplemented!(),
    }
}

pub fn day2() {
    let input = super::get_input();
    let score: u32 = input.iter().map(String::as_str).map(score).sum();
    dbg!(score);
    let score2: u32 = input.iter().map(String::as_str).map(score2).sum();
    dbg!(score2);

}
