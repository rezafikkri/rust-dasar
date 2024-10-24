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

#[test]
fn number() {
    let a: u8 = 255;
    println!("{a}");

    let b: f32 = 2345.4;
    println!("{b}");
}

#[test]
fn integer_and_float_conversion() {
    let a: i8 = 122;
    println!("{a}");

    let b: i64 = a as i64;
    println!("{b}");

    let c: i32 = b as i32;
    println!("{c}");

    let d: f32 = 123.4;
    println!("{d}");

    let e: f64 = d as f64;
    println!("{e}");

    // ex. Integer Overflow
    let f: i32 = 1000000000;
    println!("{f}");

    let g: i8 = f as i8;
    println!("{g}");
}

#[test]
fn numeric_operator() {
    let a = 10;
    let b = 12;
    let c = a*b;
    println!("{c}");
}

#[test]
fn augmented_assignment() {
    let mut a = 20;
    a -= 10;
    println!("{a}");
}

#[test]
fn boolean() {
    let a = false;
    let b: bool = true;
    println!("{a} {b}");
}
