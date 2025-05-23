use std::env;

fn to_c(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

fn to_f(c: f32) -> f32 {
    (c * 1.8) + 32.0
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_1: f32 = match args.get(1) {
        Some(value) => value.trim().parse().unwrap_or(-9999.0),
        None => -9999.0,
    };

    let input_2 = match args.get(2) {
        Some(value) => value,
        None => "X",
    };

    if input_1 == -9999.0 {
        println!("Please enter a valid number.");
        return;
    }

    if input_2.eq_ignore_ascii_case("c") {
        println!("Converting F to C");
        println!("{}F --> {}C", input_1, to_c(input_1))
    } else if input_2.eq_ignore_ascii_case("f") {
        println!("Converting C to F");
        println!("{}C --> {}F", input_1, to_f(input_1))
    } else if input_2 == "X" {
        println!("Please enter 'C' or 'F' to convert")
    }
}
