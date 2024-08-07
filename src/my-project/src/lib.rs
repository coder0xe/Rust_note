// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waiting(){}
//     }
// }

// pub fn eat_at_restaurant() {
//     crate::front_of_house::hosting::add_to_waiting();
//     front_of_house::hosting::add_to_waiting();
// }

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }

//     fn cook_order() {}
// }

// mod back_at_house {
//     pub struct Breakfast {
//         pub toast : String,
//         seasonal_fruit : String,
//     }

//     impl Breakfast {
//         pub fn summer(toast : &str) -> Breakfast {
//             Breakfast {
//                 toast : String::from(toast),
//                 seasonal_fruit : String::from("peaches"),
//             }
//         }
//     }

//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant() {
//     let mut meal = back_at_house::Breakfast::summer("Rye");
//     meal.toast = String::from("Wheat");
//     print!("I'd like {} toast please", meal.toast);
//     meal.seasonal_fruit = String::from("strawberries"); // error
// }

mod front_of_house;

pub use crate::front_of_house::hosting as dqr;
// use front_of_house::hosting;

pub fn eat_at_restaurant() {
    dqr::add_to_waiting();
    // hosting::function();
}
