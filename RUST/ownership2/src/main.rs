use std::rc::Rc;

fn main() {
    let x = return_a_string1();
    println!("{x}");
    let x = return_a_string2();
    println!("{x}");
    let x = return_a_string3();
    println!("{x}");
    let mut x = String::new();
    return_a_string4(&mut x);
    println!("{}", x);

    let v: Vec<String> = vec![String::from("Hello 5")];
    let s_ref: &String = &v[0];
    let s: String = s_ref.clone();  // Clone the string to create a new owned String
    println!("{}", s);
}

fn return_a_string1() -> String {
    let s = String::from("Hello 1");
    s
}

fn return_a_string2() -> &'static str {
    "Hello 2"
}

fn return_a_string3() -> Rc<String> {
    let s = Rc::new(String::from("Hello 3"));
    Rc::clone(&s)
}

fn return_a_string4(output: &mut String) {
    output.replace_range(.., "Hello 4");
}