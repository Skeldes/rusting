fn main() {
    let _a = 5; //Statement 
    println!("Hello World !");
    another_function(52, "Bonsooooir");

    let y = {
        let x = 3;
        x * 3}; //Ne pas mettre de ';' Le ";" indique que la ligne est une statement, or une statement ne retourne pas de valeur. 
               //Sans c'est une expression qui elle retourne une valeur
    println!("The value of y is : {} ", y);

    println!("This function return : {}", return_five());

    let x = 18;
    println!("The variable x + 1 = {}",add_one(x));
}

fn another_function(x: i32, l : &str){
    println!("The value of x is : {} the label of x is : {}", x, l);   
}

fn return_five() -> i8 {
    5
}

fn add_one(x :i32) -> i32{
    x+1
}