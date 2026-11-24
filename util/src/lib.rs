pub mod point {
    #[derive(Hash, Copy, Clone, PartialEq, Eq, Debug)]
    pub struct Point<T: Eq + Ord + Copy> {
        pub x: T,
        pub y: T,
    }

    impl<T: Eq + Ord + Copy> Point<T> {
        pub fn new(x: T, y: T) -> Self {
            Point { x, y }
        }
    }
}

pub mod direction {
    use super::point::Point;

    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Direction {
        pub fn to_offset(self) -> Point<i32> {
            match self {
                Direction::Right => Point::new(1, 0),
                Direction::Up => Point::new(0, 1),
                Direction::Left => Point::new(-1, 0),
                Direction::Down => Point::new(0, -1),
            }
        }
    }
}

pub mod regex {
    pub use lazy_static::lazy_static;
    pub use regex::Regex;
}

pub mod marker {
    /// Returns the number of bytes consumed up to and including the end of the
    /// first window of `n` pairwise-distinct lowercase letters, or `None`.
    ///
    /// Single O(len) pass: a sliding count per letter plus a running count of
    /// letters that currently appear more than once in the window.
    pub fn find_marker(input: &[u8], n: usize) -> Option<usize> {
        if n == 0 {
            return Some(0);
        }
        let mut counts = [0u8; 26];
        let mut duplicates = 0usize;
        for (i, &byte) in input.iter().enumerate() {
            let c = (byte.wrapping_sub(b'a')) as usize;
            if c >= 26 {
                return None;
            }
            counts[c] += 1;
            if counts[c] == 2 {
                duplicates += 1;
            }
            if i >= n {
                let old = (input[i - n] - b'a') as usize;
                if counts[old] == 2 {
                    duplicates -= 1;
                }
                counts[old] -= 1;
            }
            if i + 1 >= n && duplicates == 0 {
                return Some(i + 1);
            }
        }
        None
    }

    #[cfg(test)]
    mod tests {
        use super::find_marker;

        #[test]
        fn examples() {
            let cases = [
                ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
                ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
                ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
                ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
                ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
            ];
            for (input, p1, p2) in cases {
                assert_eq!(find_marker(input.as_bytes(), 4), Some(p1), "{input}");
                assert_eq!(find_marker(input.as_bytes(), 14), Some(p2), "{input}");
            }
        }

        #[test]
        fn no_marker() {
            assert_eq!(find_marker(b"aaaa", 2), None);
        }
    }
}

#[cfg(test)]
mod tests {
    /*
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    */
}
