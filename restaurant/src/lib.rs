mod front_of_house;
mod back_of_house;

mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        // using the use scope
        hosting::add_to_waitlist();

        let order1 = super::back_of_house::Appetizer::Salad;
        let order2 = super::back_of_house::Appetizer::Soup;

        // Order a breakfast in the summer with Rye toast
        let mut meal = super::back_of_house::Breakfast::summer("Rye");
        // Change our mind about what break we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't complile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");

        crate::front_of_house::hosting::add_to_waitlist();

        super::front_of_house::hosting::add_to_waitlist();
    }
}

fn deliver_order() {}