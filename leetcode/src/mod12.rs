pub fn get_count(string: &str) -> usize {
    // println!("{:?}", string.chars().collect::<Vec<char>>().iter().filter(|x| (**x == 'a' || **x == 'e' || **x == 'i' || **x == 'o' || **x =='u')).collect::<Vec<&char>>().len());
    string.chars()
          .collect::<Vec<char>>()
          .iter()
          .filter(|x| (**x == 'a' || **x == 'e' || **x == 'i' || **x == 'o' || **x =='u'))
          .collect::<Vec<&char>>()
          .len()
}
  
fn right(string: &str) -> usize {
    string.matches(|x| match x {'a'|'e'|'i'|'o'|'u' => true, _ => false}).count()
}