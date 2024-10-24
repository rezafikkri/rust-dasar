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

// type casting
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

#[test]
fn comparison_operator() {
    let a = 10 > 20;
    println!("{a}");
}

#[test]
fn boolean_operator() {
    let absen = 70;
    let nilai_akhir = 80;

    let lulus_absen = absen > 75;
    let lulus_nilai_akhir = nilai_akhir > 75;

    let lulus = lulus_absen && lulus_nilai_akhir;

    println!("{lulus}");
}

#[test]
fn char_type() {
    let char1: char = 'a';
    let char2: char = 'b';

    println!("{char1} {char2}");
}

#[test]
fn tuple() {
    let data = (10, 5.5, true);
    println!("{:?}", data);

    // println!("{}", data.0);
    // println!("{}", data.1);
    // println!("{}", data.2);

    let (a, b, _) = data;
    println!("{} {}", a, b);
}

#[test]
fn mutable_tuple() {
    let mut data = (10, 5.5, true);
    println!("{:?}", data);

    data.0 = 11;
    println!("{:?}", data.0 as f32);
}

fn unit() {
    println!("Hello");
}

#[test]
fn test_unit() {
    let result = unit(); // akan menghasilkan unit atau tuple kosong '()'
    println!("{:?}", result);
}

#[test]
fn array() {
    let arr = [12, 34, 56];
    println!("{:?}", arr);

    println!("{}", arr[0]);

    let [a, b, c] = arr;
    println!("{a}");
    println!("{b}");
    println!("{c}");
}

#[test]
fn mutable_array() {
    let mut arr = [13, 14, 15, 10];
    println!("{:?}", arr);

    arr[0] = 200;
    println!("{:#?}", arr);
}

#[test]
fn array_length() {
    let arr = [1, 2, 3, 4, 100, 99];
    println!("{}", arr.len()); // 6
}

#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 2]; 2] = [
        [1, 2],
        [3, 4],
    ];

    println!("{:?}", matrix);
    println!("{}", matrix[1][0]);
}
