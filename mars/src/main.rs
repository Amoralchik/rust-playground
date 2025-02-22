fn main() {
    let mars_weight: f32 = calculate_weight_on_mars(100.0);
    println!("Weight on mars: {}", mars_weight);
}


fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
