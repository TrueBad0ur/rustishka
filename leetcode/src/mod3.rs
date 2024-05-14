pub fn find_average(slice: &[f64]) -> f64 {
    match slice.len() {
        0 => 0.,
        n => slice.iter().sum::<f64>() / n as f64
    }
}