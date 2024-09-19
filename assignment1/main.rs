
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    
    let mut temp_f = 32.0;

    
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{:.2}°F is {:.2}°C", temp_f, temp_c);

    
    let back_to_f = celsius_to_fahrenheit(temp_c);
    println!("{:.2}°C is {:.2}°F", temp_c, back_to_f);

    for _ in 1..=5 {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        let back_to_f = celsius_to_fahrenheit(temp_c);
        println!("{:.2}°F is {:.2}°C | {:.2}°C is {:.2}°F", temp_f, temp_c, temp_c, back_to_f);
    }
}