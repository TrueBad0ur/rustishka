mod mod1;
mod mod2;
mod mod3;
mod mod4;
mod mod5;
mod mod6;
mod mod7;
mod mod8;
mod mod9;
mod mod10;

use mod2::count_positives_sum_negatives;
use mod3::find_average;
use float_eq::assert_float_eq;
use mod4::abbrev_name;
use mod5::rps;
use either::Either;
use mod6::sum_mix;
use mod7::litres;
use mod8::are_you_playing_banjo;
use mod9::points;
use mod10::square_digits;

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

fn mod7_test() {
    assert_eq!(litres(2.), 1);
    assert_eq!(litres(1.4), 0);
    assert_eq!(litres(12.3), 6);
    assert_eq!(litres(0.82), 0);
    assert_eq!(litres(11.8), 5);
    assert_eq!(litres(1787.), 893);
    assert_eq!(litres(0.), 0);
}

fn mod8_test() {
    assert_eq!(are_you_playing_banjo("Martin"), "Martin does not play banjo");
    assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
    assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
    assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
}

const ERR_MSG1: &str = "\nYour result (left) did not match the expected output (right)";
    
fn do_fixed_test(e: &[&str], expected: u32) {
    let a = &e.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    assert_eq!(points(a), expected, "{ERR_MSG1} with games = {a:?}")
}

fn mod9_test() {
    do_fixed_test(&["1:0", "2:0", "3:0", "4:0", "2:1", "3:1", "4:1", "3:2", "4:2", "4:3"], 30);
    do_fixed_test(&["1:1", "2:2", "3:3", "4:4", "2:2", "3:3", "4:4", "3:3", "4:4", "4:4"], 10);
    do_fixed_test(&["0:1", "0:2", "0:3", "0:4", "1:2", "1:3", "1:4", "2:3", "2:4", "3:4"], 0);
    do_fixed_test(&["1:0", "2:0", "3:0", "4:0", "2:1", "1:3", "1:4", "2:3", "2:4", "3:4"], 15);
    do_fixed_test(&["1:0", "2:0", "3:0", "4:4", "2:2", "3:3", "1:4", "2:3", "2:4", "3:4"], 12);
}

fn mod10_test() {
    assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
}

fn main() {
    // assert_eq!(mod1::count_sheep(&[false, true, true]), 2);
    // dotest(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15], &[10, -65]);
    // mod3_test();
    // mod4_test();
    // mod5_test();
    // mod6_test();
    // mod7_test();
    // mod8_test();
    // mod9_test();
    mod10_test();
}
