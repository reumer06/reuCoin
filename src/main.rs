struct Marker(i32); // unit struct
struct tuple(u8, u8, u8); // struct like tuple

fn main() {
    let _marker = Marker(10); // type with  one variant;
    println!("Value: {}", _marker.0);

    let info = ("Amitabh", 10, 10);
    println!("Name: {}", info.0);
    println!("Num1: {}", info.1);
    println!("Num2: {}", info.2);
}
