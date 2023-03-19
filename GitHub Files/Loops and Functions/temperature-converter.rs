fn main() {

    let x:f32 = 35.0;
    let fx = celsius_to_fahrenheit(x);
    println!("{x} celsius is {fx} fahrenheit");

    let y:f32 = -12.45;
    let cy = fahrenheit_to_celsius(y);
    println!("{y} fahrenheit is {cy} celsius");
    
}

fn celsius_to_fahrenheit(temp: f32) -> f32 {
    temp * (9.0 / 5.0) + 32.0 as f32
}

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    temp * (9.0 / 5.0) - 32.0 as f32
}
