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

#[test]
fn static_typing() {
    // let mut name = "Reza Sariful Fikri";
    let name = "Reza Sariful Fikri";
    println!("My name is {name}");

    // name = 10;
    println!("My name now is {name}");
}

#[test]
fn shadowing() {
    let name = "Reza Sariful Fikri";
    println!("My name is {name}");

    let name = 10;
    println!("My name now is {name}");
}

/* Ini adalah komentar
 * Lebih dari satu baris.
 */
#[test]
fn comment() {
    println!("Comment"); // Ini adalah comment satu baris
    println!("Comment single line");
}

#[test]
fn explicit() {
    let number: u8 = 128;
    print!("{number}");
}
