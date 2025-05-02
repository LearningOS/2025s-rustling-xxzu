// 宏必须定义在使用它之前
macro_rules! my_macro1 {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro1!(); // 正确：宏已在前面定义
}
