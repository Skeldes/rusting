use rand::Rng;
use std::io;
use std::cmp::Ordering;


struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value > 100 || value < 1 {
            panic!("Guess value must be between 1 and 100, got : {}", value)
        }

        Guess { value}
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {

    println!("Bienvenue dans le guessing game !");

    let rnd_num = rand::thread_rng().gen_range(1..101);

    loop{
        println!("votre nombre ?");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");

        println!("vous avez tapez : {}", guess);

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        match guess.value().cmp(&rnd_num){
            Ordering::Less => println!("Trop petit"),
            Ordering::Greater => println!("Trop grand !"),
            Ordering::Equal => {
                println!("Perfecto !");
                break;
            }
        }
    }
}
