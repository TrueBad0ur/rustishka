pub fn count_sheep(sheep: &[bool]) -> u8 {
  let answer = sheep.into_iter()
                        .filter(|b| **b)
                        .collect::<Vec<&bool>>()
                        .len()
                        .try_into()
                        .unwrap();
  return answer
}

fn answer(sheep: &[bool]) -> u8 {
  sheep              // take the sheep array
    .iter()          // turn it into an iterable
    .filter(|&&x| x) // filter it by taking the values in the array and returning only the true ones
    .count() as u8   // count all of the elements in the filtered array and return a u8
}