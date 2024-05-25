pub fn parse(code: &str) -> Vec<i32> {
    let mut resultValue = 0;
    let mut resultVec: Vec<i32> = Vec::new();

    code.chars().for_each(|c| match c {
        'i' => resultValue += 1,
        'd' => resultValue -= 1,
        's' => resultValue *= resultValue,
        'o' => resultVec.push(resultValue),
        _ => ()
    });
    resultVec
}