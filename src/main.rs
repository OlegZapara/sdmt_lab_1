use std::{error, io};

fn get_discriminant(a: f64, b: f64, c: f64) -> f64 {
    b.powi(2) - 4.0 * a * c
}

fn get_quadratic_equation_roots(a: f64, b: f64, c: f64) -> (Option<f64>, Option<f64>) {
    let d = get_discriminant(a, b, c);

    match d.total_cmp(&0.0) {
        std::cmp::Ordering::Less => (None, None),
        std::cmp::Ordering::Equal => (Some(-b / (2.0 * a)), None),
        std::cmp::Ordering::Greater => (
            Some((-b + d.sqrt()) / (2.0 * a)),
            Some((-b - d.sqrt()) / (2.0 * a)),
        ),
    }
}

fn get_coefficient(name: &str) -> Result<f64, Box<dyn error::Error>> {
    println!("Enter coefficient {name}: ");

    let mut a = String::new();
    io::stdin().read_line(&mut a)?;

    let a: f64 = a.trim().parse()?;

    Ok(a)
}

fn main() {
    let a: f64 = loop {
        match get_coefficient("a") {
            Ok(val) => break val,
            Err(err) => {
                println!("Error: {err}")
            }
        }
    };

    let b: f64 = loop {
        match get_coefficient("b") {
            Ok(val) => break val,
            Err(err) => {
                println!("Error: {err}")
            }
        }
    };

    let c: f64 = loop {
        match get_coefficient("c") {
            Ok(val) => break val,
            Err(err) => {
                println!("Error: {err}")
            }
        }
    };

    println!("Equation: ({a}) * x ^ 2 + ({b}) * x + ({c})");

    match get_quadratic_equation_roots(a, b, c) {
        (None, None) => println!("There are no real roots"),
        (Some(x), None) => println!("There is one real root: {x}"),
        (Some(x1), Some(x2)) => println!("There are two real roots: {x1} and {x2}"),
        _ => unreachable!("Unexpected root combination"),
    }
}
