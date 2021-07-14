mod lib1;
mod lib2;
mod lib3;
mod lib4;
mod lib5;



mod front_of_house;//使用mod关键字声明front_of_house模块，具体的定义在front_of_house.rs文件中

pub fn eat_at_restaurant() {
    //绝对路径，绝对路径必须以crate开头，因为它代码整个Module树的根节点。路径之间使用的是双冒号来表示引用
    crate::front_of_house::hosting::add_to_waitlist();
    //相对路径
    front_of_house::hosting::add_to_waitlist();

}
