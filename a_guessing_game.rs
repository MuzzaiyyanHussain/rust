use std::io;
fn main() {
    println!("Guess a number");
    println!("Guess now");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess) 
    .expect("Line cannot be recognised");

    println!("You guessed: {}", guess);
}