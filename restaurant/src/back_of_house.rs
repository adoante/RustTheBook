#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad,
}

pub struct Breakfast {
    pub toast: String,
    seasonal_fruits: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruits: String::from("peaches"),
        }
    }

    pub fn get_fruit(&self) -> &str {
        &self.seasonal_fruits
    }
}

fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
}

fn cook_order() {}
