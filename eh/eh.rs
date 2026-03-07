pub enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let somevalue = Some("safe to unwarp");
    let nonevalue: Option<&str> = None;
    println!("somevalue: {}", somevalue.expect("this will not panic"));

    println!("nonevalue: {}", nonevalue.expect("this will panic bad"));
}
