enum Message {
    Quite,
    Write(String),
    Move(i32, i32),
}

fn main() {
    let quite = Message::Quite;
    show_message(quite);

    let write = Message::Write(String::from("hi there, good morning"));
    show_message(write);

    let move_message = Message::Move(2, 3);
    show_message(move_message);
}

fn show_message(message: Message) {
    match message {
        Message::Quite => { println!("Exiting"); },
        Message::Write(message) => { println!("the message is {}",message); },
        Message::Move(x, y) => { println!("move {} meters in x direction and {} meterws in y direction", x, y); },
    }
}