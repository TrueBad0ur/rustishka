mod mod1;
mod mod2;

use mod2::count_positives_sum_negatives;

fn dotest(a: &[i32], expected: &[i32]) {
    let actual = count_positives_sum_negatives(a.to_vec());
    assert!(actual == expected, 
        "With input = {a:?}\nExpected {expected:?} but got {actual:?}")
}
fn main() {
    //assert_eq!(mod1::count_sheep(&[false, true, true]), 2);
    dotest(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15], &[10, -65]);
}
