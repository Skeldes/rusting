use std::io;

fn main() {
    let x = 5;
    let mut y = 6;
    println!("Value of x is : {}", x);

    println!("Value of y is : {}", y);

    y = 10; //Possible car y est mutable

    println!("Value of y is : {}", y);

    
    // x = 6; Impossible car x est immutable
    // println!("Value of x is : {}", x);
    

    const THREE_HOUR_IN_SECONDS : u32 = 60 * 60 * 3; //Constante, toujours immutable, typer (u32) et par convention, en majuscule

    let x = x + 1;

    {
        let x = x * 2;
        println!("Value of x : {}", x);
    }


    println!("Value of x : {}", x);

    //Scalar type
    let inte : u32 = 128_453;
    let flo : f32 = 356.85;
    let bol : bool = true;
    let cha : char = 'b';


    //Compount types

    let tup : (i32, i16) = (500,12);

    let tab = [1,2,3,4,5,6,7];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index : usize = index.trim().parse().expect("Index entered was not a number");

    let element = tab[index];

    println!("You asked this element : {}", element);
    
}
