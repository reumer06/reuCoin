// pub enum Option<T> {     // already defined;
//     Some(T),
//     None,
// }

fn main() {
    let somevalue = Some("safe to unwarp");
    let nonevalue: Option<&str> = None;
    println!("somevalue: {}", somevalue.unwrap_or_default());

    println!("nonevalue: {}", nonevalue.unwrap_or_default());
}
