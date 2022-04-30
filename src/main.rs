use std::env::{args, Args};

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'X' | 'x' => first_number * second_number,
        _ => panic!("Invalid operator used."),
    }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!(
        "{} {} {} = {}",
        first_number, operator, second_number, result
    )
}
fn main() {
    let mut args: Args = args();
    let _first: String = args.nth(1).unwrap();
    let _operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let _second: String = args.nth(0).unwrap();

    let first_number = _first.parse::<f32>().unwrap();
    let second_number = _second.parse::<f32>().unwrap();
    let result = operate(_operator, first_number, second_number);

    println!("{}", output(first_number, _operator, second_number, result));
}
