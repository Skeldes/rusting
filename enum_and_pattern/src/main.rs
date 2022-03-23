enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message {
    Quit,
    Move(i32,i32),
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message{
    fn call(&self){
        //Some stuff
    }
}



fn show_kind(ip: IpAddrKind) {
    match ip {
        IpAddrKind::V4(_,_,_,_) => println!("It's  v4 !"),
        IpAddrKind::V6(_) => println!("It's a v6 !"),
    }
}

fn main() {

    let four = IpAddrKind::V4(127,0,0,1);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum value is : {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("This work too : {}", max);
    }

    show_kind(four);
}
