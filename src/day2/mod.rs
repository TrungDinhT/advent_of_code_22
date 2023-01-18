use std::fs;

pub mod part1 {
    use super::*;

    pub fn total_score() -> u32 {
        total_score_impl("src/day2/input.txt", round_score)
    }

    pub fn round_score(opponent_play: char, my_play: char) -> u8 {
        use game::*;
        match_score(match_result(opponent_play, my_play)) + play_score(my_play)
    }
}

pub mod part2 {
    use super::*;

    pub fn total_score() -> u32 {
        total_score_impl("src/day2/input.txt", round_score)
    }

    pub fn round_score(opponent_play: char, my_play: char) -> u8 {
        use game::*;
        match my_play {
            'X' => match_score(MatchResult::LOSE) + play_score(lose_play(opponent_play)),
            'Y' => match_score(MatchResult::DRAW) + play_score(draw_play(opponent_play)),
            'Z' => match_score(MatchResult::WIN) + play_score(win_play(opponent_play)),
            _ => panic!("Unknown play {}", my_play),
        }
    }
}

mod tests {
    #[test]
    fn test_score_part1() {
        assert_eq!(
            15,
            super::total_score_impl("src/day2/input_test.txt", super::part1::round_score)
        );
    }

    #[test]
    fn test_score_part2() {
        assert_eq!(
            12,
            super::total_score_impl("src/day2/input_test.txt", super::part2::round_score)
        );
    }
}

fn total_score_impl<F>(input_file: &str, round_score: F) -> u32
where
    F: Fn(char, char) -> u8,
{
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

mod game {
    pub fn draw_play(play: char) -> char {
        match play {
            'A' => 'X',
            'B' => 'Y',
            'C' => 'Z',
            _ => panic!("Unknown play: {}", play),
        }
    }

    pub fn win_play(play: char) -> char {
        match play {
            'A' => 'Y',
            'B' => 'Z',
            'C' => 'X',
            _ => panic!("Unknown play: {}", play),
        }
    }

    pub fn lose_play(play: char) -> char {
        match play {
            'A' => 'Z',
            'B' => 'X',
            'C' => 'Y',
            _ => panic!("Unknown play: {}", play),
        }
    }

    pub fn match_score(result: MatchResult) -> u8 {
        match result {
            MatchResult::WIN => 6,
            MatchResult::DRAW => 3,
            MatchResult::LOSE => 0,
        }
    }

    pub fn play_score(play: char) -> u8 {
        match play {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("Unknown play: {}", play),
        }
    }

    pub enum MatchResult {
        WIN,
        DRAW,
        LOSE,
    }

    pub fn match_result(opponent_play: char, my_play: char) -> MatchResult {
        if draw_play(opponent_play) == my_play {
            MatchResult::DRAW
        } else {
            let win_play = win_play(opponent_play);
            if win_play == my_play {
                MatchResult::WIN
            } else {
                MatchResult::LOSE
            }
        }
    }
}
