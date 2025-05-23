// clippy3.rs
//
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // 修正拼写错误
        // my_option.unwrap(); // or handle `None` gracefully
    }

    // 直接使用数组而不是引用
    let my_arr = [-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    // 不需要调用 `resize`，直接创建空 Vec
    let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 使用 std::mem::swap 来交换值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
