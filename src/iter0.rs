pub fn test() {
    fn2();
}

fn fn2() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

use std::time;
fn fn3(r1: Result<i8, ()>) {
    
    let max_time = time::Instant::now();
    for _ in 0..1000 {
        println!("ds")
    }
}
