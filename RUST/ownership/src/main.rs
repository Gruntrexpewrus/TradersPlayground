// Created by LP
// Date: 2024-07-14
// Trade only the money you can't afford to lose
// Then go back to the mine
// And try again.
// This was coded with love <3

fn main() {
    let s = String::from("hello");
    let _s2;
    let b = false;
    if b {
      _s2 = s.clone(); //if not cloned rust does not know if s is reassigned and gives error
    }
    println!("{}", s);
  }