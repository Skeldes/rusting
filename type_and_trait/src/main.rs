

use aggregator::{Summary, Tweet};

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U>{
    x: T,
    y: U,
}


impl <T> Point<T> { //implémentation générique
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2)+self.y.powi(2)).sqrt()
    }
}




//duplicate code
// fn largest_i32(l: &[i32]) -> i32 {
//     let mut largest = l[0];
//     for &number in l {
//         if number> largest {largest = number}
//     }
//     largest
// }

// fn largest_char(list: &[char]) -> char {
//     let mut char_max = list[0];
//     for &elem in list{
//         if char_max < elem {
//             char_max = elem
//         }
//     }
//     char_max
// }

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &elem in list {
       if elem > largest { largest = elem}
    }
    largest
}

fn main() {
    let number_list = vec![15,42,5,3,6,5,4,8,45,42];

    let result = largest(&number_list);

   println!("Largest number of the list : {}", result);


    let char_list = vec!['a', 'x','z','s','h','e','f'];
    //let result = largest(&char_list);
    //println!("Largest char in list  : {}", result);


    let origine = Point{x:0, y:0};
    let pos_float = Point{x:0.4, y:1.8};
    //let pos_not_work = Point{x:2, y:2.5};
    let pos_work = Point2{x:5, y:2.5};

    println!("p.x : {}", origine.x());
    println!("distance from origine of p2 : {}", pos_float.distance_from_origin());

    let tweet = Tweet {
        username: String::from("Skeldes"),
        content: String::from(
            "My first tweet with rust !"
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet : {}", tweet.summarize());
}