// Created by LP
// Date: 2024-07-09
// Trade only the money you can't afford to lose
// Then go back to the mine
// And try again.
// This was coded with love <3

fn main() {
    let mut count: u32 = 1;

    let you = loop {
    println!("Hello, world!");
    if count == 3 {
        println!("The world has been saluted exacly {count} times. Kudos.");
        break "sucker"; //nice way to return something lmao, ; optional
    }
    count += 1;
    };
    println!("So now Hello to you, {you}.");


}
