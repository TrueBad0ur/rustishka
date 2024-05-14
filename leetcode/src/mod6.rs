use either::Either;

pub fn sum_mix(arr: &[Either<i32, String>]) -> i32 {
    let mut sum = 0;

    for elem in arr {
        match elem.to_string() {
            x => { sum += x.parse::<i32>().unwrap(); },
        }
    }

    return sum
}

fn right(arr: &[Either<i32, String>]) -> i32 {
    arr.iter().cloned().map(|e| e.left_or_else(|x| x.parse::<i32>().unwrap())).sum()
}