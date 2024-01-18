
fn main() {
    
    let mut x = 5;
    println!("The value of x is: {} ", x);
    x = x + 1;

    // inner scope
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let space = "   ";
    let space = space.len();

    let mut _spaces = "";
    _spaces = "olaaa"; // use
    println!("Space = {space}");


}
