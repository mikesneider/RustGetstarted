mod front_of_house;
/*
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
*/
//pub use crate::front_of_house as FoF;

pub fn eat_at_restaurant() {
    
    front_of_house::front_of_house();
    //hosting::add_to_waitlist();
    //add_to_waitlist();
    //add_to_waitlist();
}
