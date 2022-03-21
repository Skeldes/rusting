fn main() {
    { //s is not valid yet
        let _s = "hello"; // s is valid, it's a string literals, stored in the stack. it's immutable
        //s is valid
    } // s is not valid, out of scope
    {
        let mut s = String::from("Hello"); //this string is stored on the heap
        s.push_str(", wolrd!");
        println!("{}",s);
    }//this scope is over and the space use by s is free

    let s1 = String::from("Hello");
    let _s2 = s1; //s1 is not longer valid

   // println!("value of s1 = {}, value of s2 = {}",s1 ,s2); so we can use it like this

    let s3 = String::from("Coucou"); //s3 is valid

    take_ownership(s3); //s3 is copyed into take_ownership scope so s3 it's not longer valid

  //  println!("{}", s3); //error

  let mut s4 = String::from("Don't take me home");
  dont_take_ownership(&s4);
  println!("s4 is still here ! {}", s4);
  edit_me(&mut s4);
  println!("The new value of s4 is : {}",s4);

  {
    let r1 = &s4;
    //let mut r2 = &mut s4;  don't work
    print!("r1 value : {}", r1);
  }
  let r2 = &mut s4;

  println!(" r2 : {}",r2);


  let mut s = String::from("Hello world");
  let word = first_word(&s);
  println!("the first world of s is : {}", word);
  s.clear();
  //println!("the word is still here : {}", word);
}

fn take_ownership(s :String) {
    println!("{}", s);
}

fn dont_take_ownership(s : &String) { //take a reference to s, the ptr
    println!("{}", s);
}

fn edit_me(s : &mut String) { //we accepte only mutable references
    s.push_str(" hehe i am edited !");
}

fn first_word(s : &str) -> &str { //use &str insted of &String for more general and useful api
    let bytes = s.as_bytes(); //Convert string to bite array
    for(i, &item) in bytes.iter().enumerate() { //create a iterator for byte array with iter()
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}
