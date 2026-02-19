union MyUnion {
    p: u32,
    q: f32,
}
fn main() {
    let mut my_union = MyUnion { p: 10 };
    my_union.p = 12334553; // writing to union is safe;
    unsafe {
        // reading from union is unsafe;
        // type punning writing the data as one type and reading it with another; [very unsafe]
        println!("p : {}", my_union.p);
        println!("q : {}", my_union.q); // bunch of zeros.
    }
}
