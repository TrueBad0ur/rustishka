pub struct Son {
    pub name: String,
    pub age: u64,
}

pub fn say_hi(elem: &mut Son) {
    println!("Hi, I'm {}, and I'm {}", elem.name, elem.age);
}