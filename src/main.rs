fn hi() {
    println!("hi");
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn c_add(a: i32) -> impl Fn(i32) -> i32 {
    move |b| a + b
}

fn main() {
    let add_five = |x| add(5, x); // partial application { half filled arguments};
    println!("partial application: {}", add_five(10));

    let _result = c_add(5)(10); // currying {breaking the func into taking multiple args ;
    let add_ten = c_add(10);
    println!("currying: {}", add_ten(10));

    let u = hi;
    u();
}
