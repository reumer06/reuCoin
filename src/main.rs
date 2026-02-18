// Existential type.

fn iter() -> impl Iterator<Item = i32> {
    // RPIT  (Return Position Impl type);
    let mut i = 0;
    std::iter::from_fn(move || {
        i += 1;
        if true {
            Some(i)
        } else {
            None
        }
    })
}

//
// fn iter2() -> i32 {
//     let mut i = 0;
// }
fn main() {
    for val in iter() {
        println!("the value is {}", val);
    }
}
