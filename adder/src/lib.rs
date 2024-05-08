pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    _width: u32,
    _height: u32,
}

pub struct Guess {
    _value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { _value: value }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self._width > other._width && self._height > other._height
        }
    }
    use super::*;

    // #[test]
    // fn another() {
    //     panic!("This is a test failure.");
    // }

    #[test]
    fn explore() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            _width: 10,
            _height: 20,
        };

        let smaller = Rectangle {
            _width: 5,
            _height: 10,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
