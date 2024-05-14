mod mod1;
mod mod2;
mod mod3;

use mod2::count_positives_sum_negatives;
use mod3::find_average;
use float_eq::assert_float_eq;

fn mod3_test() {
    let input = [
        17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0,
    ];

    assert_float_eq!(
        find_average(&input),
        200.0 / 13.0,
        abs <= 1e-9, r2nd <= 4.0 * f64::EPSILON
    );

    assert_float_eq!(find_average(&[]), 0.0, abs <= 1e-9, r2nd <= 4.0 * f64::EPSILON);
}

fn dotest(a: &[i32], expected: &[i32]) {
    let actual = count_positives_sum_negatives(a.to_vec());
    assert!(actual == expected, 
        "With input = {a:?}\nExpected {expected:?} but got {actual:?}")
}
fn main() {
    //assert_eq!(mod1::count_sheep(&[false, true, true]), 2);
    // dotest(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15], &[10, -65]);
    mod3_test();

}
