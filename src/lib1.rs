mod front_of_house {

    pub mod hosting {

        pub fn add_to_waitlist() {}

        fn seat_at_table() {}

    }

    mod serving {

        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

    }

}

pub fn eat_at_restaurant() {
    //绝对路径，绝对路径必须以crate开头，因为它代码整个Module树的根节点。路径之间使用的是双冒号来表示引用
    //因为 eat_at_restaurant 函数与 front_of_house 定义于同一模块中，我们可以从 eat_at_restaurant 中引用 front_of_house。
    crate::front_of_house::hosting::add_to_waitlist();
    //相对路径，front_of_house 模块与 eat_at_restaurant 定义于同一模块，所以从 eat_at_restaurant 中开始定义的该模块相对路径是有效的。
    front_of_house::hosting::add_to_waitlist();

}
