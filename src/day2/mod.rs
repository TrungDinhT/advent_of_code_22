use std::fs;

pub fn total_score() -> u32 {
    total_score_impl("src/day2/input.txt")
}

mod tests {
    #[test]
    fn test_score() {
        assert_eq!(15, super::total_score_impl("src/day2/input_test.txt"));
    }
}

fn total_score_impl(input_file: &str) -> u32 {
    fs::read_to_string(input_file)
        .unwrap()
        .split("\n")
        .map(|round| {
            let opponent_play = round.chars().nth(0).unwrap();
            let my_play = round.chars().nth_back(0).unwrap();
            round_score(opponent_play, my_play) as u32
        })
        .sum()
}

fn round_score(opponent_play: char, my_play: char) -> u8 {
    match_score(match_result(opponent_play, my_play)) + play_score(my_play)
}

fn match_score(result: MatchResult) -> u8 {
    match result {
        MatchResult::WIN => 6,
        MatchResult::DRAW => 3,
        MatchResult::LOSE => 0,
    }
}

fn play_score(play: char) -> u8 {
    match play {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Unknown play: {}", play),
    }
}

enum MatchResult {
    WIN,
    DRAW,
    LOSE,
}

fn match_result(opponent_play: char, my_play: char) -> MatchResult {
    if equivalent_play(opponent_play) == my_play {
        MatchResult::DRAW
    } else {
        let winning_play = winning_play(opponent_play);
        if winning_play == my_play {
            MatchResult::WIN
        } else {
            MatchResult::LOSE
        }
    }
}

fn equivalent_play(play: char) -> char {
    match play {
        'A' => 'X',
        'B' => 'Y',
        'C' => 'Z',
        _ => panic!("Unknown play: {}", play),
    }
}

fn winning_play(play: char) -> char {
    match play {
        'A' => 'Y',
        'B' => 'Z',
        'C' => 'X',
        _ => panic!("Unknown play: {}", play),
    }
}
