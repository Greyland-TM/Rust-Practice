mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let customer_name = String::from("Greyland Miller");
    hosting::add_to_waitlist(&customer_name);
}
