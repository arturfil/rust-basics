
// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for item in list {
//         println!("{item}");
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

const integer = Some(5);
const float = Some(5.5);

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T, U> Point<T, U> {
    fn mixup<J, K>(self, other: Point<J, K>)  -> Point<T, K> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        println!("{item}");
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn generics_main() {
    let number_list = vec![12,32,11,8,9,7];
    let result = largest(&number_list);
    println!("{result}");

    /*
    let integer = Point {x: 5, y: 2};
    let float = Point {x: 2.5, y: 7.8};
    let mixed_types =  Point {x: 3.1, y: 2};
    */

    let interger = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}


