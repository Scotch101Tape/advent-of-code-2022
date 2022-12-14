pub mod point {
    #[derive(Hash, Copy, Clone)]
    pub struct Point<T: Eq + Ord + Copy> {
        x: T,
        y: T,
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
