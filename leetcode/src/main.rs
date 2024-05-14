mod mod1;
mod mod2;
mod mod3;
mod mod4;
mod mod5;
mod mod6;

use mod2::count_positives_sum_negatives;
use mod3::find_average;
use float_eq::assert_float_eq;
use mod4::abbrev_name;
use mod5::rps;
use either::Either;
use mod6::sum_mix;

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
fn mod4_test() {
    assert_eq!(abbrev_name("Sam Harris"), "S.H");
    assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
    assert_eq!(abbrev_name("Evan Cole"), "E.C");
    assert_eq!(abbrev_name("P Favuzzi"), "P.F");
    assert_eq!(abbrev_name("David Mendieta"), "D.M");
}

const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

fn mod5_dotest(p1: &str, p2: &str, expected: &str) {
    assert_eq!(rps(p1, p2), expected, "{ERR_MSG} with p1 = \"{p1}\", p2 = \"{p2}\"")   
}

fn mod5_test() {
    mod5_dotest("rock", "scissors", "Player 1 won!");
    mod5_dotest("scissors", "rock", "Player 2 won!");
    mod5_dotest("rock", "rock", "Draw!");
}

fn mod6_dotest(arr: &[Either<i32, String>], expected: i32) {
    let actual = sum_mix(arr);
    assert!(actual == expected, "With arr = {arr:?}\nExpected {expected} but got {actual}")
}

fn mod6_test() {
    mod6_dotest(&[Either::Left(9), Either::Left(3), Either::Right("7".to_string()), Either::Right("3".to_string())], 22);
    mod6_dotest(&[Either::Right("5".to_string()), Either::Right("0".to_string().to_string()), Either::Left(9), Either::Left(3), Either::Left(2), Either::Left(1), Either::Right("9".to_string()), Either::Left(6), Either::Left(7)], 42);
    mod6_dotest(&[Either::Right("3".to_string()), Either::Left(6), Either::Left(6), Either::Left(0), Either::Right("5".to_string()), Either::Left(8), Either::Left(5), Either::Right("6".to_string()), Either::Left(2), Either::Right("0".to_string())], 41);
    mod6_dotest(&[Either::Right("1".to_string()), Either::Right("5".to_string()), Either::Right("8".to_string()), Either::Left(8), Either::Left(9), Either::Left(9), Either::Left(2), Either::Right("3".to_string())], 45);
    mod6_dotest(&[Either::Left(8), Either::Left(0), Either::Left(0), Either::Left(8), Either::Left(5), Either::Left(7), Either::Left(2), Either::Left(3), Either::Left(7), Either::Left(8), Either::Left(6), Either::Left(7)], 61);
}

fn main() {
    // assert_eq!(mod1::count_sheep(&[false, true, true]), 2);
    // dotest(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15], &[10, -65]);
    // mod3_test();
    // mod4_test();
    // mod5_test();
    mod6_test();
}
