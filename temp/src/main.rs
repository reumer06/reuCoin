async fn xd() {
    println!("Back to the future");
}
fn main() {
    println!("Cat");
    let x = xd();
    println!("new orleans");
    futures::executor::block_on(x);
}
