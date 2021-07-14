mod back_of_house {

    //公有结构体
    pub struct Breakfast {
        //公有字段 toast
        pub toast: String,
        //私有字段 seasonal_fruit
        seasonal_fruit: String,

    }

    impl Breakfast {
        //添加关联方法
        pub fn summer(toast: &str) -> Breakfast {

            Breakfast {

                toast: String:: from(toast),

                seasonal_fruit: String:: from( "peaches"),

            }

        }

    }

}

pub fn eat_at_restaurant() {

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer( "Rye");

    // Change our mind about what bread we'd like
    meal.toast = String:: from( "Wheat");

    println!( "I'd like {} toast please", meal.toast);

}

