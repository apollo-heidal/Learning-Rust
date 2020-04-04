use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); 
    // can also use String::with_capacity(); to explicitly allocate memory

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess: {}", guess);
}
