use std::io;

fn main() {
    // println! is macro
    println!("❓ Guessing the number");
    println!("Please input you guess");

    // An associated function, function that's implimented on a type String:: in this case.
    let mut guess = String::new();

    // the & indicates this arg is a reference,
    // gives a way to let multiple parts of code access one piece of data without
    // needing to copy that data into memory multiple times.
    // references immutable by default... need the &mut
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
