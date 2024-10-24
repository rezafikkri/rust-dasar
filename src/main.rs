fn main() {
    println!("Hello, world!");
    println!("Hello, Reza!");
}

#[test]
fn test_hello() {
    println!("This is hello world!");
}

#[test]
fn variable() {
    let name = "Reza Sariful Fikri";
    println!("My name is {name}");
}

#[test]
fn variable_nutable() {
    let mut name = "Reza Sariful Fikri";
    println!("My name is {name}");

    name = "Adelina Damayanti";
    println!("My name now is {name}");
}
