use super::hosting;
use crate::back_of_house::Appetizer;
use crate::back_of_house::Breakfast;

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);
    println!("Yummy yummy in my tummy, {}!.", meal.get_fruit());

    // Won't compile b/c not public, it is private
    //meal.seasonal_fruits = String::from("blueberries");

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;

    println!("{order1:?}");
    println!("{order2:?}");

    // Absolute path
    //crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // Relative path
    //front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
