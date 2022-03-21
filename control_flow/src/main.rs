fn main() {
    let number = 3;
    if number < 5 { //with number = 2 the first 2 condition are true but rust execute only the first true statement
        println!("between 0 and 4");
    } else if number < 10 {
        println!("between 5 and 9");
    } else {
        println!("greater than 9")
    }

    // if number { We need a bool here
    //     println!("Hello error !");
    // }

    let x = if number > 5 {8} else {1};

    println!("The value of x is : {}",x );


    // 
    //       Loop statement
    //     

    // loop {
    //     println!("again !");
    // }

    let mut count = 0;
    'counting_loop: loop {
        println!("Count = {}", count);
        let mut remaining = 10;
        loop{
            println!{"remaining = {}", remaining}
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // let mut x : u8 = 1;
    // let mut count = 0;
    // let result = loop {
    //     count += 1;
    //     if x == 0 {
    //         break count;
    //     }
    //     x += 1;
    //     println!("{}", x) // when x is egal to 255 and we add 1, it's a overflow, it's not return to 0
    // };
    // println!("The value max of x is : {}", result);

    let mut counter = 0;
    while counter < 5 {
        println!("Counter = {}", counter);
        counter += 1;
    };

    let a = [1,2,3,4,5,6,7,8,9,10];
    for element in a {
        println!("{}", element);
    }
}
