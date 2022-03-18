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
    
}
