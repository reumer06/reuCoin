fn main() {
    let option: Option<i32> = Some(10); // here you have give a argument to Some; in our Case 10 otherwise this will not compile;

    let x = 5; // this is irrefutable; x will always bind to 5.
    let (a, b) = (10, 20);

    // refutable can be handled using match or switch statement; if let is for single statement.
    match option {
        Some(val) => {
            println!("value exits");
        }
        _ => println!("NONE"),
    }
    if let Some(val) = option {
        // this is refutable; if option is NONE val will not exist.
        println!("value: {val}");
    } else {
        println!("NONE");
    }
}

// Similar thing Kind of exists in C++
/*
#include <optional>
#include <iostream>

int main(){
    std::optional<int> value = 10; // Same method of printing; However this will compile if you has none to this;
    if(value.has_value()){
    int option = value.value();
    std::cout << val;
}
}
*/
