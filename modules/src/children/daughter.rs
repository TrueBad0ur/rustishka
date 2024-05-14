pub struct Daughter {
    pub name: String,
    pub age: u64,
}

pub fn say_hi(elem: &mut Daughter) {
    println!("Hi, I'm {}, and I'm {}", elem.name, elem.age);
}