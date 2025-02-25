use std::io::{self, Write};

fn main() {
    print!("Enter your weight in kilograms: ");
    io::stdout().flush().unwrap(); // displayed print before input
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    let mars_weight: f32 = calculate_weight_on_mars(weight);
    println!("Weight on mars: {}", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
