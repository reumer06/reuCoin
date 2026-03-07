// pub enum Option<T> {     // already defined;
//     Some(T),
//     None,
// }

fn main() {
    let somevalue = Some("safe to unwarp");
    let nonevalue: Option<&str> = None;
    println!("somevalue: {}", somevalue.unwrap_or("this will not panic"));

    println!("nonevalue: {}", nonevalue.unwrap_or("might panic"));
}
