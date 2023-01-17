mod day1;

fn main() {
    println!(
        "Day1:\n  The most calories are carried: {}\n  Calories carried by top 3 are: {}",
        day1::most_carried_calories(),
        day1::sum_top3_most_carried_calories(),
    );
}
