pub fn day1() {
    let mut input = super::get_input();
    input.push(String::default());

    let mut elves_calories: Vec<u32> = vec![];

    let mut most_calories: u32 = 0;
    let mut calories: u32 = 0;

    for line in input {
        if line.is_empty() {
            elves_calories.push(calories);
            if calories > most_calories {
                most_calories = calories;
            }
            calories = 0;
        } else {
            calories += line.parse::<u32>().unwrap();
        }
    }

    dbg!(most_calories);

    elves_calories.sort_unstable();
    let top3: u32 = elves_calories.iter().rev().take(3).sum();

    dbg!(top3);
}