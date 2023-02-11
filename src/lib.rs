mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub fn run(day: u8) {
    match day {
        1 => {
            println!(
                "Day1:\n  The most calories are carried: {}\n  Calories carried by top 3 are: {}",
                day1::most_carried_calories(),
                day1::sum_top3_most_carried_calories(),
            );
        }
        2 => {
            println!(
                "Day2:\n Total score if following blindly strategy guide: {}\n \
        Total score if following correctly strategy guide: {}",
                day2::part1::total_score(),
                day2::part2::total_score(),
            );
        }
        3 => {
            println!(
                "Day3:\n Sum of the priorities of wrongly shared items between 2 compartments: {}\n \
                Sum of badges priorities: {}",
                day3::part1::sum_shared_items_priorities(),
                day3::part2::sum_badges_priorities(),
            );
        }
        4 => {
            println!(
                "Day4:\n Number of assignment pairs where one range fully contains the other: {}\n \
                Number of overlapped assignment pairs: {}",
                day4::part1::nb_pairs_fully_overlapped(),
                day4::part2::nb_pairs_overlapped()
            );
        }
        5 => {
            println!(
                "Day5:\n Top of warehouse stacks after rearrangement with M9000: {} \n \
                Top of warehouse stacks after rearrangement with M9001: {} ",
                day5::part1::top_crates_after_rearrangement(),
                day5::part2::top_crates_after_rearrangement(),
            );
        }
        _ => {
            panic!("Unimplemented solution for day {}", day);
        }
    }
}
