mod day1;
mod day2;
mod day3;

fn main() {
    println!(
        "Day1:\n  The most calories are carried: {}\n  Calories carried by top 3 are: {}",
        day1::most_carried_calories(),
        day1::sum_top3_most_carried_calories(),
    );

    println!(
        "Day2:\n Total score if following blindly strategy guide: {}\n Total score if following correctly strategy guide: {}",
        day2::part1::total_score(),
        day2::part2::total_score(),
    );

    println!(
        "Day3: Sum of the priorities of wrongly shared items between 2 compartments: {}",
        day3::sum_shared_items_priorities()
    )
}
