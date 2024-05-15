pub fn positive_sum(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for i in slice {
        if i > &0 {
            sum += i;
        }
    }
    sum
}

pub fn right(slice: &[i32]) -> i32 {
   //println!("{:?}", slice.iter().filter(|x| **x==3).collect::<Vec<&i32>>());
    slice.iter().filter(|x| x.is_positive()).sum()
}