// if we use pub before a struct definiion, we make the struct public, but the struct's fields will be still private

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    // also, because back_of_house has a private field, the struct needs to provide a public associated function that constructs an instance of Breakfast(aka summer)
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // in contrast, if we make an enum public, all of its variants are then public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // order a breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change our mind about what bread we'd like
    meal = back_of_house::Breakfast::summer("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // won't compile
    // meal.seasonal_fruit = String::from("berries");
}
