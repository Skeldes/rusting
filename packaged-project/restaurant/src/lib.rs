mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist(); //crate is the root of any project

    //relative path
    front_of_house::hosting::seat_at_table();
}