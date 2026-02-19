// Closures

fn main() {
    let x = 42;
    let u = || println!("{x}");
    u();

    let z = |x: i32| x * 2; // in most cases type cannot be inferred; specify them directly.
    println!("{}", z(7));

    let y = |(a, b): (i32, i32)| -> i32 { a + b }; // return type can also be specified.
    y((10, 23));
}
