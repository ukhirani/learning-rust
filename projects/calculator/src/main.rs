use std::io;

fn main() {
    println!("UMANG'S CALCULATOR");
    println!("==================");

    println!("Please enter the operation : add, subtract, multiply and divide");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to execute");

    let mut num1 = String::new();
    let mut num2 = String::new();
    let result: i32;

    println!("Enter First Number : "); // TODO: somehow use print! instead of println buffer issue
    io::stdin()
        .read_line(&mut num1)
        .expect("Enter Valid Number");

    print!("Enter Second Number : ");
    io::stdin()
        .read_line(&mut num2)
        .expect("Enter Valid Number");

    let num1 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let num2 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    result = match guess.as_str().trim() {
        "add" => num1 + num2,
        "subtract" => num1 - num2,
        "multiply" => num1 * num2,
        "divide" => {
            if num2 == 0 {
                0 // return 0 if tried to divide by 0
            } else {
                num1 / num2
            }
        }
        _ => 123, // default case
    };

    print!(
        "Here is your result for {} operation : {} ",
        guess.trim(),
        result
    );
}
