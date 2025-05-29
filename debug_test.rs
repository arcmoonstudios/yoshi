use std::error::Error;
use std::fmt::{Display, Formatter};
use yoshi::{Yoshi, YoshiKind};

#[derive(Debug)]
struct MyCustomError;

impl Display for MyCustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "a custom error occurred")
    }
}

impl Error for MyCustomError {}

fn main() {
    let boxed_err = Box::new(MyCustomError);
    let yoshi_err = Yoshi::foreign(boxed_err);
    println!("Yoshi error display: {}", yoshi_err);
    println!("Contains 'My custom error': {}", format!("{}", yoshi_err).contains("My custom error"));
    println!("Contains 'a custom error occurred': {}", format!("{}", yoshi_err).contains("a custom error occurred"));
}
