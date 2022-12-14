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
