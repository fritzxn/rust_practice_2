//enumerations and pattern matching

enum Message {
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        //write blah blah
    }
}
fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
    
    let some_char = Some(5);
    let some_number = Some('e');
    let absent_number: Option<i32> = None;


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
   }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}








}