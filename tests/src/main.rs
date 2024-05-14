#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

fn main() {
    let x = ("1", 12, "1231", '1');
    println!("{:?}", x.3);
}
