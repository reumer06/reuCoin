enum Message {
    Quit,                       // plain name
    Move { x: i32, y: i32 },    // struct like
    Write(String),              // tuple like
    ChangeColor(i32, i32, i32), // tuple like
}

fn main() {
    let some_message = Message::Write(String::from("quill!"));

    if let Message::Write(ref some_text) = some_message {
        // conditional peak
        println!("{some_text} was written!");
    }

    if matches!(some_message, Message::Write(_)) {
        // conditional check
        println!("bill");
    }

    let Message::Write(ref some_text) = some_message else {
        // guard clause
        return;
    };

    let some_text: String = if let Message::Write(ref text) = some_message {
        // transformation
        text.clone()
    } else {
        unreachable!()
    };

    // println!("{some_text}");
    match some_message {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("values of x and y are {x} and {y}"),
        Message::Write(text) => println!("Text message: {text}"),
        Message::ChangeColor(r, g, b) => println!("change the rgb values to: {r}, {g} , {b}"),
    }
}
