struct User{
    active: bool,
    username: String,
    email: String,
    sing_in_count : u64, 
}

struct Color(i32,i32,i32);
struct Point(i32, i32, i32); //struct tuples

struct Nothing;


fn build_user(email: String, username: String) -> User {
    User{
        email,
        username, //Init shorthand
        active : true,
        sing_in_count: 0
    }
}


fn main() {
    let user1 = User{
        email : String::from("thomas.feuilletin@gmail.com"),
        username : String::from("Skeldes"),
        active : false,
        sing_in_count : 8
    };

    let user2 = build_user(String::from("otheremail@mail.com"), String::from("secondUser"));
    let user3 = User{
        email : String::from("anotherEmail@email.com"),
        ..user2
    };

    println!("User1 email : {}\nUser2 email : {}\nUser3 email : {}", user1.email, user2.email, user3.email);
   // println!("User1 username : {}\nUser2 username : {}\nUser3 username : {}", user1.username, user2.username, user3.username); user2 it's not longer valid :( rip
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let reason_to_live = Nothing;

}