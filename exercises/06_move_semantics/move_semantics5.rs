#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char { // 传递 data 的引用，即借用，因为 get_char 只需要读取数据，不需要修改
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) { //  传递 data 的所有权，因为 string_uppercase 需要对数据进行修改
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}
