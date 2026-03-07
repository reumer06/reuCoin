pub enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let value1 = Some("safe to unwarp");
    let none_value: Option<&str> = None;
    println!("value1: {}", value1.unwrap());
    println!("none value : {}", none_value.unwarp()); // crash
}
