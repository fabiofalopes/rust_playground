use std::io;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {

    // Scalar Types  
    // - integers, 
    // - floating-point numbers, 
    // - Booleans, 
    // - characters

    println!("Scalar Types: 
    - integers, 
    - floating-point numbers, 
    - Booleans, 
    - characters ");

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let multiplication = 4*30;
    let quocient = 56.7 / 32.2;
    let truncated = -5 / 3 ;
    let remainer = 43 % 5 ;

    println!("Sum = {sum}");
    print!("type of sum = ");
    print_type_of(&sum);
    println!("difference = {difference} ");
    println!("multiplication = {multiplication} ");
    println!("quocient = {quocient} ");
    println!("truncated = {truncated} ");
    println!("remainer = {remainer} ");


    let t = true;

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("boolean = {t} ");

    println!("c = {c} ");
    println!("z = {z} ");
    println!("heart_eyed_cat = {heart_eyed_cat} ");

    // Compound Types
    
    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred = {five_hundred} ");
    println!("six_point_four = {six_point_four} ");
    println!("one = {one} ");

    // Array Type 

    let a = [1 , 2 , 3 , 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let first = a[0];
    let second = a[1];
    
    println!("first = {first} ");
    println!("second = {second} ");

    // Invalid Array Element Access 
    // Will compile and will print correct results but will panic if index inputed is out of bounds

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

    println!("The value of the element at index {index} is: {element}");


}
