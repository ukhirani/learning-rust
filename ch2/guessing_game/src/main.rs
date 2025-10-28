use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    /*
        The purpose of these Result types is to encode error-handling informa-
    tion. Values of the Result type, like values of any type, have methods defined
    on them. An instance of io::Result has an expect method that you can call.
    If this instance of io::Result is an Err value, expect will cause the program to
    crash and display the message that you passed as an argument to expect. If
    the read_line method returns an Err, it would likely be the result of an error
    coming from the underlying operating system.
        */

    // ok so basically read_line returns the type RESULT which are enums with variants (ok OR err)
    // if ok --> give the code the value that the user unimplemented!
    // if err --> the result type err has method expect that you can send message through

    println!("You guessed: {}", guess);
}
