use std::cmp::PartialOrd;
use std::fmt::Debug;

// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//
//     for number in list {
//         if number > largest {
//             largest = number;
//         }
//     }
//
//     largest
// }

// Generics are not that difficult really and are super useful. I am really stating to love
// rust for how explicite and safe it is.
struct RandomData<T, U, C> {
    x: T,
    _y: U,
    _z: C,
}

impl<T, U, C> RandomData<T, U, C> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest<T: PartialOrd + Debug>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        println!("{:?}", item);
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let num_list = vec![30, 25, 164, 11, 69];

    let largest_int = largest(&num_list);
    println!("The largest number is: {}", largest_int);

    let char_list = vec!['a', 't', 'v', 'm'];

    let largest_char = largest(&char_list);
    println!("The largest char is: {}", largest_char);

    let random_data_1 = RandomData {
        x: 1,
        _y: 6.5,
        _z: "d",
    };

    let x = random_data_1.get_x();
    println!("The value for x is: {}: ", x);

    let point = Point { x: 3.4, y: 2.5 };
    let distance = point.distance_from_origin();
    println!("The distance between points is: {}", distance);
}
