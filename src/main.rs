fn main() {
    let code = "hellowoody";
    let len = calculate_len(code);
    println!("{len}");
}

fn calculate_len(s: &str) -> usize{
    s.len()
}
