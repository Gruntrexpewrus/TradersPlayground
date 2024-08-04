fn main() {
    let x = 10;
    let y = &x;  // `y` is a reference to `x`

    println!("Value of x: {}", x);
    println!("Value via y: {}", *y);  // Dereferencing `y` to access `x`
    
    let mut z = 10;
    let z_ref = &mut z;  // `z_ref` is a mutable reference to `z`
    *z_ref += 10;  // Dereference and modify `z` through `z_ref`

    println!("Updated value of z: {}", z);

    let name = vec!["Sir".to_string(), "Leo".to_string()];
    let new_name;
    new_name = stringify_name_with_title(&name);
    println!("Your extended name is: {}", new_name);

    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("{s_ref}!");

    let v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v[0].clone();
    s.push('!!');
    println!("{s}");

    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);

    


    
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}