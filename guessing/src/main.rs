use rand::Rng;
use std::io;
use std::cmp::Ordering;

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

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&rnd_num){
            Ordering::Less => println!("Trop petit"),
            Ordering::Greater => println!("Trop grand !"),
            Ordering::Equal => {
                println!("Perfecto !");
                break;
            }
        }
    }
}
