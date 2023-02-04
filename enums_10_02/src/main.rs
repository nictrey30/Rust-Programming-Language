// all the variants are grouped together under the Message type
enum Message {
    // Quit - has no data associated with it at all
    Quit,
    // Move - includes an anonymous struct inside it
    Move { x: i32, y: i32 },
    // Write - includes a single String
    Write(String),
    // ChangeColor - includes i32 values
    changeColor(i32, i32, i32),
}

// unit struct
// struct QuitMessage;
// struct MoveMessage {
//   x: i32,
//   y: i32
// }
// tuple struct
// struct WriteMessage(String);
// tuple struct
// struct ChangecolorMessage(i32, i32, i32);

// but if we used diffrent structs, which each have their own type, we ouldn't easily define a function to take any of these kind of messages as we could with the Message enum, which is a single type
impl Message {
    fn call(&self) {
        // method defined here
    }
}
fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
