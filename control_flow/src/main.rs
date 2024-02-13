fn main() {
    let number :i32 = 6;

    if number < 3 {
        println!("Menor que 3");
    }
    else {
       println!("Maior ou igual a 3"); 
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //Rust only executes the block for the first true condition, 
    //and once it finds one, it doesn’t even check the rest

    // neste caso 6 também divide por 3 
}
