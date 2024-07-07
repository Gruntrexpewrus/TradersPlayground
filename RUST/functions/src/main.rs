fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello, world!");
    another_function();
    function_x(5);
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1 //if you add ";" here it become sa statement and gives error
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of five is: {x}");

    println!("Adding one to {x} gives: {}", plus_one(x));

}

/*Rust doesn’t care where you define your functions, 
only that they’re defined somewhere in a scope that can be seen by the caller.*/
fn another_function() {
    println!("Another function!");
}

fn function_x(x: i32) {
    println!("The value of x is: {x}");
}
//In function signatures, you must declare the type of each parameter
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
