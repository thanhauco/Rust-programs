// Import necessary modules
use std::f64::consts::PI;

fn main() {
    // Define the limits of integration and the number of intervals
    let a = 0.0;  // Lower limit of integration
    let b = 1.0;  // Upper limit of integration
    let n = 1000; // Number of intervals (must be even for Simpson's Rule)

    // Call the simpson's rule function
    let result = simpson_integrate(a, b, n, |x| x.powi(2)); // Integral of x^2 from 0 to 1

    // Print the result
    println!("The integral of x^2 from {} to {} is approximately: {}", a, b, result);
}

// Define the function that performs Simpson's Rule
fn simpson_integrate<F>(a: f64, b: f64, n: usize, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    if n % 2 != 0 {
        panic!("Number of intervals (n) must be even for Simpson's Rule.");
    }

    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);

    // Apply Simpson's Rule
    for i in 1..n {
        let x = a + i as f64 * h;
        if i % 2 == 0 {
            sum += 2.0 * f(x);
        } else {
            sum += 4.0 * f(x);
        }
    }

    (h / 3.0) * sum
}