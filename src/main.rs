fn print_type1<T: std::fmt::Debug>(u : T){
    println!("{:?}",u);
}
fn print_type2(u : impl std::fmt::Debug){
    println!("{:?}",u);
}

fn main() {
    let x = (0..25)
        .map(|x| x * 2)
        .filter(|x| x % 3  == 0)
        .collect();
    print_type1::<Vec<i32>>(x);
    print_type2::<Vec<i32>>(x); // 0 generic parameters.
}