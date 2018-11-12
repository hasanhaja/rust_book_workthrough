pub fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp-32.0)*(5.0/9.0)
}

pub fn celsius_to_fahrenheit(temp: f64) -> f64 {
    temp*(9.0/5.0) + 32.0
}
