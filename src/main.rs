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

fn main() {}
