use std::io;

const ONE_HOUR_IN_SECONDS: u32 = 60*60;

fn main() {
    let mut x = 5; //writing let x = 5; would make the conde compile with error because you try to reassign an immutable variable
    println!("The value of x is {x}");

    x = 6;
    println!("The value of x is {x}");
    //Rust’s naming convention for constants is to use all uppercase with underscores between words.
    const THREE_HOURS_IN_SECONDS : u32 = 60*60*3; //only constant expression not computable on runtime allowed for const

    println!("In 3 hours there are {} seconds.", THREE_HOURS_IN_SECONDS);
    println!("In 1 hour there are {} seconds.", ONE_HOUR_IN_SECONDS);

    let y = 10;
    //shadowing: you can also change type (you can't if you don't reuse let and mutate)
    let y = y + 10;
    {
        let y = y + 112;
        println!("The value of y in the inner scope is {y}");
    }
    println!("But the inner scope does not change the real y that is {y}");

    // division
    let quotient = 56.7 / 32.2;
    println!("Proper division 56.7 / 32.2 gives {quotient}");
    let truncated = -5 / 3; // Results in -1 since the types are integers
    println!("Truncated division -5/3 gives {truncated}");

    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation and SINGLE QUOTES
    let heart_eyed_cat = '😻';
    println!("Cute cat {heart_eyed_cat}");

    //let mut x : u8 = 0;
    //x = x - 1;
    //println!("x unsigned below 0: {x}"); returns panic or 255

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, _y, _z) = tup;
    println!("The tuple is ({}, {}, {})", tup.0,tup.1,tup.2);
    println!("The first value of the tuple is {x}");

    //arrays have fixed lenght, not flexible like vectors
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _same_values = [3; 5];

    println!("Please enter an array index.");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    //the program will panic with index out of bounds or give error if not a number inputted
    println!("The value of the element at index {index} is: {element}");
}
