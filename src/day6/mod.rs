use criterion::Criterion;
use std::fs;

pub mod part1 {
    pub fn find_marker_pos() -> usize {
        let buffer = super::fs::read_to_string("src/day6/input.txt").unwrap();
        super::implementation::find_packet_marker_pos_impl(&buffer)
    }
}

pub mod part2 {
    pub fn find_marker_pos() -> usize {
        let buffer = super::fs::read_to_string("src/day6/input.txt").unwrap();
        super::implementation::find_message_marker_pos_impl(&buffer)
    }
}

pub mod implementation {
    pub fn find_packet_marker_pos_impl(buffer: &str) -> usize {
        impl2::find_marker_pos_impl(buffer, 4)
    }

    pub fn find_message_marker_pos_impl(buffer: &str) -> usize {
        impl2::find_marker_pos_impl(buffer, 14)
    }

    pub mod impl1 {
        pub fn find_marker_pos_impl(buffer: &str, block_len: usize) -> usize {
            for end in block_len..buffer.len() {
                if !contains_duplicate(&buffer[end - block_len..end]) {
                    return end;
                }
            }

            panic!("Cannot find signal marker in this buffer");
        }

        fn contains_duplicate(slice: &str) -> bool {
            let mut dictionary = Alphabet::new();

            for bit in slice.chars().map(super::letter_lookup_bit) {
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

    // !!! This implementation is theoretically faster, because the algorithm does twice less steps than the implementation above
    // Instead of verifying if the sliding window contains a duplicated letter each time, we only add 1 new letter and remove
    // the starting letter of previous sliding window. Then we check if the new set of letters are all distinct.
    // However, when benchmarking with small block_len, this is much slower than the above implementation. This is probably due to the fact that
    // it uses array and memory access instead of bit operations. However, when the block_len increases, this implementation will scale better.
    pub mod impl2 {
        use super::letter_lookup_bit;

        pub fn find_marker_pos_impl(buffer: &str, block_len: usize) -> usize {
            let (mut dict, mut nb_bits_set) = Alphabet::init_with_buffer(&buffer[..block_len - 1]);
            let mut start_sliding = 0;
            for (idx, c) in buffer[block_len - 1..].chars().enumerate() {
                let bit = letter_lookup_bit(c);
                if !dict.put(bit, true) {
                    nb_bits_set = nb_bits_set + 1;
                    if nb_bits_set == block_len {
                        return idx + block_len;
                    }
                }

                let start_sliding_char = buffer.chars().nth(start_sliding).unwrap();
                let start_sliding_bit = letter_lookup_bit(start_sliding_char);
                dict.put(start_sliding_bit, false);
                if dict.at(start_sliding_bit) == 0 {
                    nb_bits_set = nb_bits_set - 1;
                }

                start_sliding = start_sliding + 1;
            }

            panic!("Cannot find signal marker in this buffer");
        }

        struct Alphabet {
            dict: [u8; 26],
        }

        impl Alphabet {
            fn new() -> Self {
                let dict = [0; 26];
                Self { dict }
            }

            fn at(&self, idx: usize) -> u8 {
                self.dict[idx]
            }

            fn put(&mut self, bit: usize, val: bool) -> bool {
                let prev = self.dict[bit];
                self.dict[bit] = if val {
                    self.dict[bit] + 1
                } else {
                    self.dict[bit] - 1
                };
                prev > 0
            }

            fn init_with_buffer(buffer: &str) -> (Self, usize) {
                let mut dict = Alphabet::new();
                let nb_bits_preset: usize = buffer
                    .chars()
                    .map(|c| {
                        let bit = letter_lookup_bit(c);
                        dict.put(bit, true) as usize
                    })
                    .sum();
                let nb_bits_set = buffer.len() - nb_bits_preset;
                (dict, nb_bits_set)
            }
        }
    }

    fn letter_lookup_bit(c: char) -> usize {
        c as usize - 'a' as usize
    }
}

#[cfg(test)]
mod tests {
    fn test_impl<F>(testsuite: &[(&str, usize)], f: F)
    where
        F: Fn(&str) -> usize,
    {
        for (buffer, ref_result) in testsuite {
            assert_eq!(f(buffer), *ref_result, "In buffer: {}", buffer);
        }
    }

    #[test]
    fn test_part1() {
        const TEST_SUITE: [(&str, usize); 5] = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        test_impl(
            &TEST_SUITE,
            super::implementation::find_packet_marker_pos_impl,
        );
    }

    #[test]
    fn test_part2() {
        const TEST_SUITE: [(&str, usize); 5] = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];
        test_impl(
            &TEST_SUITE,
            super::implementation::find_message_marker_pos_impl,
        );
    }
}
