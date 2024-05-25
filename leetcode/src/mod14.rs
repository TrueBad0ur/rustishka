static mut repeat_times: u64 = 0;

pub fn persistence(num: u64) -> u64 {
    unsafe { repeat_times = 0 };
    if num.checked_ilog10().unwrap_or(0) + 1 == 1 {
        return 0
    }

    let x = count(num);
    unsafe { repeat_times }
}

fn number_to_vec(n: u64) -> Vec<u64> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}

fn count(num: u64) -> u64 {
    if num.checked_ilog10().unwrap_or(0) + 1 == 1 {
        return num
    } else {
        unsafe { repeat_times += 1 };
        count(number_to_vec(num).iter().fold(1, |acc, &x| acc * x))
    }
    
}