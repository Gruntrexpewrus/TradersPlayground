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

    //now let's use a reference
    let m1 = String::from("Hello World");
    //References are non-owning pointers, because they do not own the data they point to.
    greet(&m1);
    let _s = format!("{}", m1);

    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    *x += 1;                 // *x on the left-side modifies the heap value,
                            //     so x points to the value 2

    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value

    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;    // so only one dereference is needed to read it
  }

  fn greet(g1: &String) {
    println!("{}", g1)
  }