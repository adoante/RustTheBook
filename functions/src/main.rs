fn main() {
    println!("Hello, world!");

    another_function(69);

    print_labeled_measurement(69, 'f');

    let x = five();

    println!("The value of x is: {x}");

    println!("{}", plus_one(x));
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurment is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
