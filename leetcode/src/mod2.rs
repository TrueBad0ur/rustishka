pub fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    let positive: Vec<i32> = input.clone()
                                  .into_iter()
                                  .filter(|elem| *elem>0)
                                  .collect();

    let negative: Vec<i32> = input.clone()
                                  .into_iter()
                                  .filter(|elem| *elem<0)
                                  .collect();

    let positive_answer: i32 = positive.len().try_into().unwrap();
    let negative_answer: i32 = negative.iter().map(|&i| i as i32).sum();

    let answer_array = Vec::from([positive_answer, negative_answer]);
    if input.clone().is_empty() {
        return vec![]
    }
    return answer_array
}

pub fn right(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return vec![];
    }
  
      input.iter().fold(vec![0, 0], |mut acc, &x| {
          if x > 0 {
          acc[0] += 1;
        } else {
          acc[1] += x;
        }
        acc
      })
}