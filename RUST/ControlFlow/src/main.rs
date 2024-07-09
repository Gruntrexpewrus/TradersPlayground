use std::io;

fn main() {
    let mut number = String::new();

    println!("Please input a number:");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line."); //the program will crash if err is returned and not return error atm

    let number: u32 = number.trim().parse().expect("Failed to parse number");
    if number < 5 {
        println!("Number is less than 5.");
    } else if number == 5 {
        println!("Number is 5.");
    }
    else {
        println!("Number is greater than 5.");
    }
    //I can also do it with match
    let condition = number==5;
    //nice way to define a double type variable
    enum NumberOrString {
        Number(i32),
        Text(&'static str),
    }

    let number_bool = if condition {
        NumberOrString::Number(5)
    } else {
        NumberOrString::Text("not five")
    };
    
    match number_bool {
        NumberOrString::Number(num) => println!("It's number {}", num),
        NumberOrString::Text(text) => println!("It's number {}", text),
    };
}
