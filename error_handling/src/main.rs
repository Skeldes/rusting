use std::{
    env,
    fs::{
        File,
        self,
    },
    io::{
        ErrorKind,
        self,
    },
};


fn main() {

    env::set_var("RUST_BACKTRACE", "0");

    //panic!("AAAAAAAAAAAAAAAAAAh"); keep calm

    // let v = vec![1,2,3,4];

    // v[99];

    //let f = File::open("hello.txt");
    //v1
    // let f = match f {
    //     Ok(file) => file,
    //     Err(why) => panic!("Probleme opening the file {:?}", why)
    // };

    //v2
    // let f = match f {
    //     Ok(file) => file,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt"){
    //             Ok(fc) => fc,
    //             Err(why) => panic!("Problem creating file {:?}", why)
    //         },
    //         other => panic!("Probleme opening file : {:?}", other)
    //     }
    // };

    //v3
    let f = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating file {:?}", error)
            })
        }else {
            panic!("Probleme opening file: {:?}", error)
        }
    });

    //let f2 = File::open("hello2.txt").unwrap(); //return file or call panic! macro
    let f3 = File::open("hello2.txt").expect("This is a error message, fail to open hello2.txt"); // return file or call panic! macro with a custom error msg

}


fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
    //let mut s = String::new();
   // File::open(path)?.read_to_string(&mut s)?;// the ? operator return Err(why) if the resultat of this code is not OK
    

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(error) => return Err(error)
    // };

    

    //Ok(s)

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(why) => Err(why),
    // }
}