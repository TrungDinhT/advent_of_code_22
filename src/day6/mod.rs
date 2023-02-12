pub mod part1 {
    pub fn find_marker_pos(buffer: &str) -> usize {
        impl1::find_marker_pos(buffer)
    }

    mod impl1 {
        use super::super::helpers;

        pub fn find_marker_pos(buffer: &str) -> usize {
            for end in 4..buffer.len() {
                if !contains_duplicate(&buffer[end - 4..end]) {
                    return end;
                }
            }

            panic!("Cannot find signal marker in this buffer");
        }

        fn contains_duplicate(slice: &str) -> bool {
            let mut dictionary = Alphabet::new();

            for bit in slice.chars().map(helpers::letter_lookup_bit) {
                let prev = dictionary.put(bit);
                if prev {
                    return true;
                }
            }

            false
        }

        pub struct Alphabet {
            dict: u32, // u32 has 32 bits, it's more than enough for English alphabet of 26 letters
        }

        impl Alphabet {
            pub fn new() -> Self {
                Self { dict: 0 } // init all the alphabet as unset
            }

            pub fn put(&mut self, bit: usize) -> bool {
                let prev_bit_val = (self.dict >> bit) & 0x1;
                self.dict = self.dict | (1 << bit);
                prev_bit_val == 1
            }
        }
    }
}

mod helpers {
    pub fn letter_lookup_bit(c: char) -> usize {
        c as usize - 'a' as usize
    }
}

#[cfg(test)]
mod tests {
    const TEST_SUITE: [(&str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    #[test]
    fn test_part1() {
        use super::part1::find_marker_pos;
        for (buffer, ref_result) in TEST_SUITE {
            assert_eq!(find_marker_pos(buffer), ref_result, "In buffer: {}", buffer);
        }
    }
}
