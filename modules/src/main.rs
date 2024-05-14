pub mod children;

pub mod local;
use local::Local;

fn main() {
    let son = children::son::Son {
        name: "Andy".to_string(),
        age: 18,
    };

    let daughter = children::daughter::Daughter {
        name: "Kate".to_string(),
        age: 20,
    };

    let local_struct = Local {
        somevar1: "aaa".to_string(),
        somevar2: 333,
    };

    son.say_hi();
    daughter.say_hi();
    local_struct.say_hi();

    println!("Hello, world!");
}
