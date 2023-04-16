mod front_of_house;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_wait_list();

    // relative path
    front_of_house::hosting::add_to_wait_list();
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant2() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // fields are private by default as well
    // meal.seasonal_fruit = String::from("Wheat");
    meal.toast = String::from("Wheat");

    // when enum is pub, all of the items are pub as well
    let order1 = back_of_house::Appetizer::Soup;
    let order1 = back_of_house::Appetizer::Salad;
}

// use use
use crate::back_of_house::Appetizer;
use crate::back_of_house::Breakfast;

use rand::Rng;

pub fn eat_at_restaurant3() {
    let rand_num = rand::thread_rng().gen_range(1..=100);

    let mut meal = Breakfast::summer("Rye");

    // fields are private by default as well
    // meal.seasonal_fruit = String::from("Wheat");
    meal.toast = String::from("Wheat");

    // when enum is pub, all of the items are pub as well
    let order1 = Appetizer::Soup;
    let order1 = Appetizer::Salad;
}
