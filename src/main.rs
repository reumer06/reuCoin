fn print1(u : impl std::fmt::Debug){
    println!("{:?}",u);
}

fn main() {
    let x = (0..10)
        .map(|x|x * 10)
        .filter(|x| x % 5 == 0)
        .collect::<Vec<i32>>();
    print1(x);
}