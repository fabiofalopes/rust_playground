fn main() {
    println!("Hello, world!");
    another_function(14);
    print_labeled_measurement(5,'h');

    // new scope is an expression
    let y = {
        let x = 3;
        x + 1
        // If we end x + 1 with semicolon then this is a statment and does
        // not return a value
    };

    println!("The value of y is: {y}");

}

fn another_function(x : i32){
    println!("The value of x is {x} ");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}