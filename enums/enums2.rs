// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.


#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Quit,
    Echo,
    Move,
    ChangeColor,
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move,
        Message::Echo,
        Message::ChangeColor,
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
